package api

import (
	m "github.com/Andorr/word-bank/api/middleware"
	"github.com/Andorr/word-bank/api/quiz"
	"github.com/Andorr/word-bank/api/word"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type Server struct {
	addr  string
	dbURI string
	e     *echo.Echo
}

type ServerOptions struct {
	Addr  string
	DBURI string
	Token string
}

type Controller interface {
	Routes(e *echo.Echo)
}

func NewServer(options ServerOptions) (*Server, error) {
	e := echo.New()

	e.Pre(middleware.RemoveTrailingSlash())

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORS())
	e.Use(m.AuthWithSingleToken(options.Token))

	e.HideBanner = true

	// Initialize wordbank
	wb, err := wordbank.New(options.DBURI)
	if err != nil {
		return nil, err
	}

	// Initialize controllers
	wordCtrl := &word.WordController{
		WB: wb,
	}
	wordCtrl.Routes(e)

	quizCtrl := &quiz.QuizController{
		WB: wb,
	}
	quizCtrl.Routes(e)

	return &Server{
		addr:  options.Addr,
		dbURI: options.DBURI,
		e:     e,
	}, nil
}

func (s *Server) Run() error {
	return s.e.Start(s.addr)
}
