package wordbank

import (
	"os"
	"testing"
	"wordbank/pkg/wordbank/models"

	"github.com/google/uuid"
	"github.com/stretchr/testify/suite"
)

type TestWBCreateWordSuite struct {
	suite.Suite
	wb *WordBank
}

func (suite *TestWBCreateWordSuite) SetupTest() {
	// Setup test
	wb, err := NewWithPG(os.Getenv("TEST_DB_URI"))
	if err != nil {
		suite.T().Fatalf("Error in SetupTest: %s", err)
		return
	}
	suite.wb = wb
}

func (suite *TestWBCreateWordSuite) TearDownTest() {
	// Tear down test
}

func (suite *TestWBCreateWordSuite) TestInsertWord() {
	// Insert word

	word := &models.Word{
		Value: "test",
		Class: models.WordClassNoun,
		Tags:  []string{},
		Translations: []*models.Translation{
			{ID: uuid.New(), Value: "test"},
		},
	}

	if err := suite.wb.Word.InsertWord(nil, word); err != nil {
		suite.FailNowf("Error inserting word", err.Error())
		return
	}

	if word.ID == nil {
		suite.FailNowf("Word ID is nil", "")
		return
	}
	suite.Assert().NotEqual(uuid.Nil, *word.ID)

	word2, err := suite.wb.Word.GetWord(nil, *word.ID)
	if err != nil {
		suite.FailNowf("Error getting word", err.Error())
		return
	}
	suite.Assert().Equal(word, word2)
}

func TestWordBankCreateWord(t *testing.T) {
	suite.Run(t, new(TestWBCreateWordSuite))
}
