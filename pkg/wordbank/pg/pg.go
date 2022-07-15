package pg

import (
	"context"
	"database/sql"
	"wordbank/pkg/wordbank/models"

	"github.com/google/uuid"
)

type PgDBStore struct {
	DB *sql.DB
}

func (c PgDBStore) InsertWord(ctx context.Context, word *models.Word) error {
	c.DB.Exec("INSERT INTO words (value, type, tags, translations, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
		word.Value, word.Type, word.Tags, word.Translations, word.CreatedAt, word.UpdatedAt)
	return nil
}

func (c PgDBStore) QueryWords(ctx context.Context, word models.WordQueryOptions) (*models.PageResult[models.Word], error) {
	// c.DB.Query("SELECT * FROM words WHERE id = $1", id)
	return nil, nil
}

func (c PgDBStore) UpdateWord(ctx context.Context, updateOptions models.WordUpdateOptions) (*models.Word, error) {
	return nil, nil
}

func (c PgDBStore) DeleteWord(ctx context.Context, id uuid.UUID) error {
	return nil
}

func (c PgDBStore) GetWordsByIds(ctx context.Context, ids []uuid.UUID) ([]*models.Word, error) {
	return nil, nil
}

func (c PgDBStore) RandomWords(ctx context.Context, count int) ([]*models.Word, error) {
	return nil, nil
}
