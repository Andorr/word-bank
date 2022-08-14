package pgutil

import "github.com/jmoiron/sqlx"

func RunTx(db *sqlx.DB, fn func(tx *sqlx.Tx) error) error {
	tx, err := db.Beginx()
	if err != nil {
		return err
	}
	defer tx.Rollback()
	err = fn(tx)
	if err != nil {
		return err
	}
	return tx.Commit()
}
