package pg

import (
	"database/sql"
	"log"
	"wordbank/pkg/util/arrayutil"
	"wordbank/pkg/wordbank/models"
	pgmodels "wordbank/pkg/wordbank/pg/models"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/lib/pq"
)

type PgDBStore struct {
	DB *sqlx.DB
}

func NewDBStore(db *sqlx.DB) *PgDBStore {
	return &PgDBStore{
		DB: db,
	}
}

// ----- WORDS ------
func (c *PgDBStore) InsertWord(word *models.Word) error {
	pgWord := pgmodels.PgWordFrom(word)

	err := c.DB.
		QueryRowx("INSERT INTO words (value, class, tags, translations) VALUES ($1, $2, $3, $4) RETURNING *",
			pgWord.Value, pgWord.Class, pq.Array(pgWord.Tags), pq.Array(pgWord.Translations)).
		StructScan(&pgWord)

	if err == nil {
		pgWord.IntoWord(word)
	}
	return err
}

func (c *PgDBStore) QueryWords(options models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Word], error) {

	qb := NewQuery("words as w").
		Column("w.*").
		CrossJoin("UNNEST(translations) as t")

	if options.Query != nil {
		qb = qb.WhereOr("value SIMILAR TO '%?%'", *options.Query)
		qb = qb.WhereOr("t.value SIMILAR TO '%?%'", *options.Query)
	}
	if options.Class != nil {
		qb = qb.WhereOr("class = ?", *options.Class)
	}
	if options.Tags != nil {
		qb = qb.WhereOr("tags @> ?", pq.Array(options.Tags))
	}
	if options.Word != nil {
		qb = qb.WhereOr("value = ?", options.Word)
	}

	countQb := qb.Count("*")

	if pagination != nil {
		qb = qb.Limit(pagination.Limit).Offset(pagination.Page * pagination.Limit)
	}

	query, params := qb.Build()
	words := make([]pgmodels.PgWord, 0)
	err := c.DB.Select(&words, query, params...)
	if err != nil {
		return nil, err
	}

	query, params = countQb.Build()
	var count int
	err = c.DB.QueryRowx(query, params...).Scan(&count)
	if err != nil {
		return nil, err
	}

	return &models.PageResult[models.Word]{
		Total: count,
		Page:  pagination.Page,
		Count: len(words),
		Results: arrayutil.Map(words, func(word pgmodels.PgWord) models.Word {
			var result models.Word
			return *word.IntoWord(&result)
		}),
	}, nil
}

func (c *PgDBStore) UpdateWord(updateOptions models.WordUpdateOptions) (*models.Word, error) {
	return nil, nil
}

func (c *PgDBStore) DeleteWord(id uuid.UUID) error {
	return nil
}

func (c *PgDBStore) GetWordsByIds(ids []uuid.UUID) ([]*models.Word, error) {
	var words []*pgmodels.PgWord
	query, args, err := sqlx.In("SELECT * FROM words WHERE id::text IN (?)", ids)
	if err != nil {
		return nil, err
	}
	query = c.DB.Rebind(query)
	err = c.DB.Select(&words, query, args...)
	if err != nil {
		return nil, err
	}

	return arrayutil.Map(words, func(w *pgmodels.PgWord) *models.Word {
		var result models.Word
		return w.IntoWord(&result)
	}), err
}

func (c *PgDBStore) RandomWords(count int) ([]*models.Word, error) {
	return nil, nil
}

// ------ FOLDERS ------
func (c *PgDBStore) InsertFolder(folder *models.Folder) error {
	return nil
}

func (c *PgDBStore) QueryFolders(folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Folder], error) {
	return nil, nil
}

func (c *PgDBStore) UpdateFolder(updateOptions models.FolderUpdateOptions) (*models.Folder, error) {
	return nil, nil
}

func (c *PgDBStore) DeleteFolder(id uuid.UUID) error {
	return nil
}

func (c *PgDBStore) GetFolder(id uuid.UUID) (*models.Folder, error) {
	return nil, nil
}

func (c *PgDBStore) GetFolderContent(id uuid.UUID) (*models.FolderContent, error) {
	return nil, nil
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
