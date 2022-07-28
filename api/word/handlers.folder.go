package word

import (
	"net/http"

	"github.com/Andorr/word-bank/internal/echoutil"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

func (ctrl *WordController) QueryFolders(c echo.Context) error {
	options, err := echoutil.BindFolderQueryOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind request: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidBody, err.Error())
	}

	paginationOptions, err := echoutil.BindPaginationOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind pagination options: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidQueryOptions, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		folders, err := ctrl.WB.Word.QueryFolders(ctx, *options, paginationOptions)
		if err != nil {
			c.Logger().Errorf("failed to query folders: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		c.JSON(200, folders)
		return nil
	})
}

func (ctrl *WordController) GetFolder(c echo.Context) error {
	id, err := echoutil.BindParamUUID(c, "id")
	if err != nil {
		c.Logger().Errorf("failed to bind folder: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidUUID, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		folder, err := ctrl.WB.Word.GetFolder(ctx, *id)
		if err != nil {
			c.Logger().Errorf("failed to get folder: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		content, err := ctrl.WB.Word.GetFolderContent(ctx, *id)
		if err != nil {
			c.Logger().Errorf("failed to get folder content: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		response := struct {
			Data    *models.Folder
			Content *models.FolderContent
		}{
			Data:    folder,
			Content: content,
		}

		c.JSON(200, response)
		return nil
	})

}

func (ctrl *WordController) CreateFolder(c echo.Context) error {
	type request struct {
		Name   string      `json:"name"`
		Parent *uuid.UUID  `json:"parent"`
		Words  []uuid.UUID `json:"words"`
	}
	var req request
	if err := c.Bind(&req); err != nil {
		c.Logger().Errorf("failed to bind request: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidBody, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		folder := models.Folder{
			Name:   req.Name,
			Parent: req.Parent,
			Words:  req.Words,
		}
		err := ctrl.WB.Word.InsertFolder(ctx, &folder)
		if err != nil {
			c.Logger().Errorf("failed to create folder: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		c.JSON(200, &folder)
		return nil
	})
}

func (ctrl *WordController) UpdateFolder(c echo.Context) error {
	options, err := echoutil.BindFolderUpdateOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind request: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidBody, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		folder, err := ctrl.WB.Word.UpdateFolder(ctx, *options)
		if err != nil {
			c.Logger().Errorf("failed to update folder: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		c.JSON(200, &folder)
		return nil
	})
}

func (ctrl *WordController) DeleteFolder(c echo.Context) error {
	id, err := echoutil.BindParamUUID(c, "id")
	if err != nil {
		c.Logger().Errorf("failed to bind folder: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidUUID, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		err := ctrl.WB.Word.DeleteFolder(ctx, *id)
		if err != nil {
			c.Logger().Errorf("failed to delete folder: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		c.Response().Status = http.StatusNoContent
		return nil
	})
}
