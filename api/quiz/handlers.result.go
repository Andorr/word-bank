package quiz

import (
	"github.com/Andorr/word-bank/api/word"
	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/internal/echoutil"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

type QuizController struct {
	WB *wordbank.WordBank
}

type QuizMode = string

const (
	QuizModeNormal  QuizMode = "Normal"
	QuizModeEndless QuizMode = "Endless"
)

func (ctrl *QuizController) InitQuiz(c echo.Context) error {
	type QuizOptions struct {
		Mode  QuizMode `json:"mode"`
		Words struct {
			Folders []uuid.UUID `json:"folders"`
			Count   *int        `json:"count"`
		} `json:"words"`
	}

	var options QuizOptions
	if err := c.Bind(&options); err != nil {
		return echoutil.ErrBadRequest(word.ErrCodeInvalidBody, err.Error())
	}

	return ctrl.WB.RunTx(func(tx *wordbank.WordBankContext) error {

		var words []*models.Word
		if options.Words.Folders != nil {

			folders, err := ctrl.WB.Word.QueryFolders(tx, models.FolderQueryOptions{
				IDs: options.Words.Folders,
			}, nil)
			if err != nil {
				return echoutil.ToHTTPError(err, c)
			}

			wordIds := arrayutil.Flatten(arrayutil.Map(folders.Results, func(f *models.Folder) []uuid.UUID {
				return f.Words
			}))

			words, err = ctrl.WB.Word.GetWordsByIDs(tx, wordIds)
			if err != nil {
				return echoutil.ToHTTPError(err, c)
			}
		} else {
			var defaultCount int = 16
			if options.Words.Count == nil {
				options.Words.Count = &defaultCount
			}

			ws, err := ctrl.WB.Word.RandomWords(tx, *options.Words.Count)
			if err != nil {
				return echoutil.ToHTTPError(err, c)
			}
			words = ws
		}

		type Quiz struct {
			ID      uuid.UUID      `json:"id"`
			Words   []*models.Word `json:"words"`
			Options QuizOptions    `json:"options"`
		}

		c.JSON(200, Quiz{
			ID:      uuid.New(),
			Words:   words,
			Options: options,
		})
		return nil
	})
}

func (ctrl *QuizController) InsertQuizResult(c echo.Context) error {
	quizResult := new(models.QuizResult)
	if err := c.Bind(quizResult); err != nil {
		return echoutil.ErrBadRequest(word.ErrCodeInvalidBody, err.Error())
	}

	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		return echoutil.ToHTTPError(err, c)
	}

	if err := ctrl.WB.Quiz.InsertQuizResult(ctx, quizResult); err != nil {
		return echoutil.ToHTTPError(err, c)
	}

	c.JSON(200, quizResult)
	return nil
}
