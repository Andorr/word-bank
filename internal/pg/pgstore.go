package pg

import (
	"context"
	"database/sql"
	"log"

	pgmodels "github.com/Andorr/word-bank/internal/pg/models"
	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/lib/pq"
)

const PgContextKey = "__pgContext"

type PgDBStore struct {
	DB *sqlx.DB
}

type PgContext struct {
	conn Queryer
}

func (store *PgDBStore) Tx() (*sqlx.Tx, error) {
	tx, err := store.DB.Beginx()
	if err != nil {
		return nil, err
	}
	return tx, nil
}

func (store *PgDBStore) CommitContext(ctx context.Context) error {
	tx, ok := ctx.Value(PgContextKey).(*sqlx.Tx)
	if !ok {
		return nil
	}
	return tx.Commit()
}

func (store *PgDBStore) CloseContext(ctx context.Context) error {
	db, ok := ctx.Value(PgContextKey).(*sqlx.DB)
	if ok {
		return db.Close()
	}
	return nil
}

func NewDBStore(connectionString string) (*PgDBStore, error) {
	db, err := sqlx.Connect("postgres", connectionString)
	if err != nil {
		return nil, err
	}

	return &PgDBStore{
		DB: db,
	}, nil
}

func (c *PgDBStore) NewContext(ctx context.Context) (*PgContext, error) {
	tx, err := c.DB.Beginx()
	return &PgContext{
		conn: tx,
	}, err
}

type Queryer interface {
	sqlx.Ext
	sqlx.ExecerContext
	sqlx.PreparerContext
	sqlx.QueryerContext
	sqlx.Preparer

	GetContext(context.Context, interface{}, string, ...interface{}) error
	SelectContext(context.Context, interface{}, string, ...interface{}) error
	Get(interface{}, string, ...interface{}) error
	MustExecContext(context.Context, string, ...interface{}) sql.Result
	PreparexContext(context.Context, string) (*sqlx.Stmt, error)
	QueryRowContext(context.Context, string, ...interface{}) *sql.Row
	Select(interface{}, string, ...interface{}) error
	QueryRow(string, ...interface{}) *sql.Row
	PrepareNamedContext(context.Context, string) (*sqlx.NamedStmt, error)
	PrepareNamed(string) (*sqlx.NamedStmt, error)
	Preparex(string) (*sqlx.Stmt, error)
	NamedExec(string, interface{}) (sql.Result, error)
	NamedExecContext(context.Context, string, interface{}) (sql.Result, error)
	MustExec(string, ...interface{}) sql.Result
	NamedQuery(string, interface{}) (*sqlx.Rows, error)
}

var _ Queryer = (*sqlx.DB)(nil)
var _ Queryer = (*sqlx.Tx)(nil)

// ----- WORDS ------
func (c *PgDBStore) InsertWord(ctx context.Context, word *models.Word) error {

	pgWord := pgmodels.PgWordFrom(word)

	err := c.driver(ctx).
		QueryRowx("INSERT INTO words (value, class, tags, translations) VALUES ($1, $2, $3, $4) RETURNING *",
			pgWord.Value, pgWord.Class, pq.Array(pgWord.Tags), pq.Array(pgWord.Translations)).
		StructScan(&pgWord)

	if err == nil {
		pgWord.IntoWord(word)
	}
	return err
}

func (c *PgDBStore) QueryWords(ctx context.Context, options models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], error) {

	qb := NewQuery("words as w").
		Column("w.*").
		CrossJoin("UNNEST(translations) as t")

	if options.Query != nil {
		qb = qb.WhereOr("w.value SIMILAR TO ?", "%"+*options.Query+"%")
		qb = qb.WhereOr("t.value SIMILAR TO ?", "%"+*options.Query+"%")
	}
	if options.Class != nil {
		qb = qb.WhereOr("w.class = ?", *options.Class)
	}
	if options.Tags != nil {
		qb = qb.WhereOr("w.tags @> ?", pq.Array(options.Tags))
	}
	if options.Word != nil {
		qb = qb.WhereOr("w.value = ?", options.Word)
	}

	countQb := qb.Count("*")

	var page = 1
	if pagination != nil && pagination.NotEmpty() {
		qb = qb.Limit(pagination.Limit).Offset(pagination.Page * pagination.Limit)
		page = pagination.Page
	}

	query, params := qb.Build()
	words := make([]pgmodels.PgWord, 0)
	err := c.driver(ctx).Select(&words, query, params...)
	if err != nil {
		return nil, err
	}

	query, params = countQb.Build()
	var count int
	err = c.driver(ctx).QueryRowx(query, params...).Scan(&count)
	if err != nil {
		return nil, err
	}

	return &models.PageResult[*models.Word]{
		Total: count,
		Page:  page,
		Count: len(words),
		Results: arrayutil.Map(words, func(word pgmodels.PgWord) *models.Word {
			return word.ToWord()
		}),
	}, nil
}

func (c *PgDBStore) UpdateWord(ctx context.Context, updateOptions models.WordUpdateOptions) (*models.Word, error) {

	qb := Update("words")

	if updateOptions.Word != nil {
		qb = qb.Set("value", *updateOptions.Word)
	}
	if updateOptions.Class != nil {
		qb = qb.Set("class", *updateOptions.Class)
	}
	if updateOptions.Tags != nil {
		qb = qb.Set("tags", pq.Array(updateOptions.Tags))
	}
	if updateOptions.Translations != nil {
		qb = qb.Set("translations", pq.Array(arrayutil.Map(updateOptions.Translations, func(t *models.Translation) pgmodels.PgTranslation {
			return pgmodels.PgTranslation{
				ID:  t.ID,
				Val: t.Value,
			}
		})))
	}

	qb = qb.Where("id = ?", updateOptions.ID).
		Set("updated_at", "now()").
		Returning("*")

	query, params := qb.Build()

	var word pgmodels.PgWord
	err := c.driver(ctx).QueryRowx(query, params...).
		StructScan(&word)

	return word.ToWord(), err
}

func (c *PgDBStore) DeleteWord(ctx context.Context, id uuid.UUID) error {
	query, params := Delete("words").Where("id = ?", id).Build()

	_, err := c.driver(ctx).Exec(query, params...)
	return err
}

func (c *PgDBStore) GetWordsByIds(ctx context.Context, ids []uuid.UUID) ([]*models.Word, error) {
	var words []*pgmodels.PgWord
	query, args, err := sqlx.In("SELECT * FROM words WHERE id::text IN (?)", ids)
	if err != nil {
		return nil, err
	}
	query = c.driver(ctx).Rebind(query)
	err = c.driver(ctx).Select(&words, query, args...)
	if err != nil {
		return nil, err
	}

	return arrayutil.Map(words, func(w *pgmodels.PgWord) *models.Word {
		return w.ToWord()
	}), err
}

func (c *PgDBStore) RandomWords(ctx context.Context, count int) ([]*models.Word, error) {
	var words []*pgmodels.PgWord
	err := c.driver(ctx).Select(&words, "SELEC * FROM words ORDER BY RANDOM() LIMIT ?", count)
	if err != nil {
		return nil, err
	}

	return arrayutil.Map(words, func(w *pgmodels.PgWord) *models.Word {
		return w.ToWord()
	}), nil
}

// ------ FOLDERS ------
func (c *PgDBStore) InsertFolder(ctx context.Context, folder *models.Folder) error {
	f := pgmodels.PgFolderFrom(folder)

	err := c.driver(ctx).
		QueryRowx("INSERT INTO folders (name, parent, words) VALUES ($1, $2, $3) RETURNING *",
			f.Name, f.Parent, f.Words).
		StructScan(&f)

	if err == nil {
		f.IntoFolder(folder)
	}
	return err

}

func (c *PgDBStore) QueryFolders(ctx context.Context, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], error) {

	qb := NewQuery("folders as f")

	if folder.Query != nil {
		qb = qb.WhereOr("f.name SIMILAR TO ?", "%"+*folder.Query+"%")
	}
	if folder.Parent != nil {
		qb = qb.WhereOr("f.parent = ?", *folder.Parent)
	}
	if folder.Words != nil {
		qb = qb.WhereOr("f.words @> ?", pq.Array(folder.Words))
	}

	countQb := qb.Count("*")

	if pagination != nil && pagination.NotEmpty() {
		qb = qb.Limit(pagination.Limit).Offset(pagination.Page * pagination.Limit)
	}

	query, params := qb.Build()
	folders := make([]pgmodels.PgFolder, 0)
	err := c.driver(ctx).Select(&folders, query, params...)
	if err != nil {
		return nil, err
	}

	query, params = countQb.Build()
	var count int
	err = c.driver(ctx).QueryRowx(query, params...).Scan(&count)
	if err != nil {
		return nil, err
	}

	return &models.PageResult[*models.Folder]{
		Total: count,
		Page:  pagination.Page,
		Count: len(folders),
		Results: arrayutil.Map(folders, func(f pgmodels.PgFolder) *models.Folder {
			return f.ToFolder()
		}),
	}, nil
}

func (c *PgDBStore) UpdateFolder(ctx context.Context, updateOptions models.FolderUpdateOptions) (*models.Folder, error) {

	var folder pgmodels.PgFolder

	tx := c.driver(ctx)

	err := tx.QueryRowx("SELECT * FROM folders WHERE id = ?", updateOptions.ID).StructScan(&folder)
	if err != nil {
		return nil, err
	}

	qb := Update("folder")
	if updateOptions.Name != nil {
		qb = qb.Set("name", *updateOptions.Name)
	}
	if updateOptions.Parent != nil {
		qb = qb.Set("parent", *updateOptions.Parent)
	}
	if len(updateOptions.Add) > 0 || len(updateOptions.Remove) > 0 {
		words := folder.Words
		if len(updateOptions.Add) > 0 {
			words = append(words, updateOptions.Add...)
		}
		if len(updateOptions.Remove) > 0 {
			words = arrayutil.Difference(words, updateOptions.Remove)
		}

		qb = qb.Set("words", pq.Array(words))
	}

	qb = qb.Where("id = ?", updateOptions.ID).
		Set("updated_at", "now()").
		Returning("*")

	query, params := qb.Build()
	err = tx.QueryRowx(query, params...).
		StructScan(&folder)

	if err != nil {
		return nil, err
	}

	return folder.ToFolder(), nil
}

func (c *PgDBStore) DeleteFolder(ctx context.Context, id uuid.UUID) error {
	query, params := Delete("folders").Where("id = ?", id).Build()

	_, err := c.driver(ctx).Exec(query, params...)
	return err

}

func (c *PgDBStore) GetFolder(ctx context.Context, id uuid.UUID) (*models.Folder, error) {
	var folder pgmodels.PgFolder
	err := c.driver(ctx).QueryRowx("SELECT * FROM folders WHERE id = ?", id).StructScan(&folder)
	if err != nil {
		return nil, err
	}

	return folder.ToFolder(), nil
}

func (c *PgDBStore) driver(ctx context.Context) Queryer {
	value := ctx.Value(PgContextKey)
	tx, ok := value.(*sqlx.Tx)
	if ok {
		return tx
	}
	// conn, ok := value.(*sqlx.Conn)
	// if ok {
	// 	return conn
	// }

	return c.DB
}

type QueryLogger struct {
	queryer sqlx.Queryer
	logger  *log.Logger
}

func (p *QueryLogger) Query(query string, args ...interface{}) (*sql.Rows, error) {
	p.logger.Printf(query, args...)
	return p.queryer.Query(query, args...)
}

func (p *QueryLogger) Queryx(query string, args ...interface{}) (*sqlx.Rows, error) {
	p.logger.Printf(query, args...)
	return p.queryer.Queryx(query, args...)
}

func (p *QueryLogger) QueryRowx(query string, args ...interface{}) *sqlx.Row {
	p.logger.Printf(query, args...)
	return p.queryer.QueryRowx(query, args...)
}
