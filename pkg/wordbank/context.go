package wordbank

import (
	"context"
	"time"
)

type WordBankContext struct {
	onCommit   func(ctx context.Context) error
	onRollback func(ctx context.Context) error
	key, value any
}

func (*WordBankContext) Deadline() (deadline time.Time, ok bool) {
	return
}

func (*WordBankContext) Done() <-chan struct{} {
	return nil
}

func (*WordBankContext) Err() error {
	return nil
}

func (ctx *WordBankContext) Value(key any) any {
	if key == ctx.key {
		return ctx.value
	}
	return nil
}

func (ctx *WordBankContext) commit() error {
	if ctx.onCommit != nil {
		return ctx.onCommit(ctx)
	}
	return nil
}

func (ctx *WordBankContext) rollback() error {
	if ctx.onRollback != nil {
		return ctx.onRollback(ctx)
	}
	return nil
}
