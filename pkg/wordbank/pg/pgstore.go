package pg

import (
	"database/sql"
	"log"
	"wordbank/pkg/util/arrayutil"
	"wordbank/pkg/wordbank/models"
	pgmodels "wordbank/pkg/wordbank/pg/models"
	"wordbank/pkg/wordbank/pg/utils"

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

	if pagination != nil && pagination.NotEmpty() {
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
	err := c.DB.QueryRowx(query, params...).
		StructScan(&word)

	return word.ToWord(), err
}

func (c *PgDBStore) DeleteWord(id uuid.UUID) error {
	query, params := Delete("words").Where("id = ?", id).Build()

	_, err := c.DB.Exec(query, params...)
	return err
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
		return w.ToWord()
	}), err
}

func (c *PgDBStore) RandomWords(count int) ([]*models.Word, error) {
	var words []*pgmodels.PgWord
	err := c.DB.Select(&words, "SELEC * FROM words ORDER BY RANDOM() LIMIT ?", count)
	if err != nil {
		return nil, err
	}

	return arrayutil.Map(words, func(w *pgmodels.PgWord) *models.Word {
		return w.ToWord()
	}), nil
}

// ------ FOLDERS ------
func (c *PgDBStore) InsertFolder(folder *models.Folder) error {
	f := pgmodels.PgFolderFrom(folder)

	err := c.DB.
		QueryRowx("INSERT INTO folders (name, parent, words) VALUES ($1, $2, $3) RETURNING *",
			f.Name, f.Parent, f.Words).
		StructScan(&f)

	if err == nil {
		f.IntoFolder(folder)
	}
	return err

}

func (c *PgDBStore) QueryFolders(folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Folder], error) {

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
	err := c.DB.Select(&folders, query, params...)
	if err != nil {
		return nil, err
	}

	query, params = countQb.Build()
	var count int
	err = c.DB.QueryRowx(query, params...).Scan(&count)
	if err != nil {
		return nil, err
	}

	return &models.PageResult[models.Folder]{
		Total: count,
		Page:  pagination.Page,
		Count: len(folders),
		Results: arrayutil.Map(folders, func(f pgmodels.PgFolder) models.Folder {
			return *f.ToFolder()
		}),
	}, nil
}

func (c *PgDBStore) UpdateFolder(updateOptions models.FolderUpdateOptions) (*models.Folder, error) {

	var folder pgmodels.PgFolder
	err := utils.RunTx(c.DB, func(tx *sqlx.Tx) error {

		err := tx.QueryRowx("SELECT * FROM folders WHERE id = ?", updateOptions.ID).StructScan(&folder)
		if err != nil {
			return err
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
		return tx.QueryRowx(query, params...).
			StructScan(&folder)
	})
	if err != nil {
		return nil, err
	}

	return folder.ToFolder(), nil
}

func (c *PgDBStore) DeleteFolder(id uuid.UUID) error {
	query, params := Delete("folders").Where("id = ?", id).Build()

	_, err := c.DB.Exec(query, params...)
	return err

}

func (c *PgDBStore) GetFolder(id uuid.UUID) (*models.Folder, error) {
	var folder pgmodels.PgFolder
	err := c.DB.QueryRowx("SELECT * FROM folders WHERE id = ?", id).StructScan(&folder)
	if err != nil {
		return nil, err
	}

	return folder.ToFolder(), nil
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
