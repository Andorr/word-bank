package wordbank

import (
	"context"
	"os"
	"testing"

	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
	"github.com/stretchr/testify/suite"
)

type TestWBSuite struct {
	suite.Suite
	wb *WordBank

	words []*models.Word
}

func (suite *TestWBSuite) SetupTest() {

	// Setup test
	wb, err := New(os.Getenv("WORDBANK_TEST_DB_URI"))
	if err != nil {
		suite.T().Fatalf("Error in SetupTest: %s", err)
		return
	}
	suite.wb = wb
}

func (suite *TestWBSuite) TearDownTest() {
	// Tear down test
}

func (suite *TestWBSuite) TestInsertAndGetWord() {
	// Insert word
	word := &models.Word{
		Value: "test",
		Class: models.WordClassNoun,
		Tags:  []string{},
		Translations: []*models.Translation{
			{ID: uuid.New(), Value: "test"},
		},
	}

	ctx, err := suite.wb.NewContext(context.Background())
	if err != nil {
		suite.FailNowf("Error creating context", err.Error())
		return
	}

	if err := suite.wb.Word.InsertWord(ctx, word); err != nil {
		suite.FailNowf("Error inserting word", err.Error())
		return
	}

	if word.ID == nil {
		suite.FailNowf("Word ID is nil", "")
		return
	}
	suite.Assert().NotEqual(uuid.Nil, *word.ID)

	word2, err := suite.wb.Word.GetWord(ctx, *word.ID)
	if err != nil {
		suite.FailNowf("Error getting word", err.Error())
		return
	}
	suite.Assert().Equal(word, word2)

	suite.words = append(suite.words, word)
}

func (suite *TestWBSuite) TestWBQueryWords() {

	ctx, err1 := suite.wb.NewContext(context.Background())
	if err1 != nil {
		suite.FailNowf("Error creating context", err1.Error())
		return
	}

	query := "test"
	class := models.WordClassNoun
	words, err2 := suite.wb.Word.QueryWords(ctx, models.WordQueryOptions{
		Query: &query,
		Class: &class,
	}, &models.PaginationOptions{})
	if err2 != nil {
		suite.T().Logf("Error: %+v", err2)
		suite.FailNowf("Error querying words", err2.Error())
		return
	}
	if suite.Assert().NotNil(words) {
		for _, word := range words.Results {
			suite.Assert().Equal(query, word.Value)
			suite.Assert().Equal(class, word.Class)
		}
	}
}

func (suite *TestWBSuite) TestInsertAndGetFolder() {
	ctx, err := suite.wb.NewContext(context.Background())
	if err != nil {
		suite.FailNowf("Error creating context", err.Error())
		return
	}

	word := &models.Word{
		Value: "test",
		Class: models.WordClassNoun,
		Tags:  []string{},
		Translations: []*models.Translation{
			{ID: uuid.New(), Value: "test"},
		},
	}
	if err := suite.wb.Word.InsertWord(ctx, word); err != nil {
		suite.FailNowf("Error inserting word", err.Error())
		return
	}
	if word.ID == nil {
		suite.FailNowf("Word ID is nil", "")
		return
	}
	suite.words = append(suite.words, word)

	// Insert folder
	folder := &models.Folder{
		Name:   "test",
		Parent: nil,
		Words: []uuid.UUID{
			*word.ID,
		},
	}

	if err := suite.wb.Word.InsertFolder(ctx, folder); err != nil {
		suite.FailNowf("Error inserting folder", err.Error())
		return
	}

	if folder.ID == nil {
		suite.FailNowf("Folder ID is nil", "")
		return
	}
	suite.Assert().NotEqual(uuid.Nil, *folder.ID)

	word2, err := suite.wb.Word.GetFolder(ctx, *folder.ID)
	if err != nil {
		suite.FailNowf("Error getting folder", err.Error())
		return
	}
	suite.Assert().Equal(folder, word2)
}

func (suite *TestWBSuite) TestWBQueryFolders() {
	ctx, err1 := suite.wb.NewContext(context.Background())
	if err1 != nil {
		suite.FailNowf("Error creating context", err1.Error())
		return
	}

	word := &models.Word{
		Value: "random-test-name",
		Class: models.WordClassNoun,
		Tags:  []string{},
		Translations: []*models.Translation{
			{ID: uuid.New(), Value: "test"},
		},
	}
	if err := suite.wb.Word.InsertWord(ctx, word); err != nil {
		suite.FailNowf("Error inserting word", err.Error())
		return
	}
	if word.ID == nil {
		suite.FailNowf("Word ID is nil", "")
		return
	}
	suite.words = append(suite.words, word)

	// Insert folder
	folder := &models.Folder{
		Name:   "test",
		Parent: nil,
		Words: []uuid.UUID{
			*word.ID,
		},
	}

	if err := suite.wb.Word.InsertFolder(ctx, folder); err != nil {
		suite.FailNowf("Error inserting folder", err.Error())
		return
	}

	wordID := word.ID
	folders, err2 := suite.wb.Word.QueryFolders(ctx, models.FolderQueryOptions{
		Words: []uuid.UUID{*wordID},
	}, &models.PaginationOptions{})
	if err2 != nil {
		suite.T().Logf("Error: %+v", err2)
		suite.FailNowf("Error querying words", err2.Error())
		return
	}
	if suite.Assert().NotNil(folders) {
		if len(folders.Results) == 0 {
			suite.FailNowf("No folders found", "")
			return
		}

		for _, folder := range folders.Results {
			suite.T().Logf("Folder: %+v", folder)
			suite.Contains(folder.Words, *wordID)
		}
	}
}

func TestWordBank(t *testing.T) {
	suite.Run(t, new(TestWBSuite))
}
