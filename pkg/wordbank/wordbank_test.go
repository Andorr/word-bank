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
}

func (suite *TestWBSuite) SetupTest() {

	// Setup test
	wb, err := NewWithPG(os.Getenv("WORDBANK_TEST_DB_URI"))
	if err != nil {
		suite.T().Fatalf("Error in SetupTest: %s", err)
		return
	}
	suite.wb = wb
}

func (suite *TestWBSuite) TearDownTest() {
	// Tear down test
}

func (suite *TestWBSuite) TestInsertWord() {
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
}

func (suite *TestWBSuite) TestWBQueryWords() {

	ctx, err := suite.wb.NewContext(context.Background())
	if err != nil {
		suite.FailNowf("Error creating context", err.Error())
		return
	}

	query := "test"
	class := models.WordClassNoun
	words, err := suite.wb.Word.QueryWords(ctx, models.WordQueryOptions{
		Query: &query,
		Class: &class,
	}, &models.PaginationOptions{})
	if err != nil {
		suite.FailNowf("Error querying words", err.Error())
		return
	}
	if suite.Assert().NotNil(words) {
		for _, word := range words.Results {
			suite.Assert().Equal(query, word.Value)
			suite.Assert().Equal(class, word.Class)
		}
	}
}

func TestWordBankCreateWord(t *testing.T) {
	suite.Run(t, new(TestWBSuite))
}
