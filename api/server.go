package api

import (
	"github.com/Andorr/word-bank/api/word"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/labstack/echo/v4"
)

type Server struct {
	Addr  string
	DBURI string
	e     *echo.Echo
}

type ServerOptions struct {
	Addr  string
	DBURI string
}

type Controller interface {
	Routes(e *echo.Echo)
}

func NewServer(options ServerOptions) (*Server, error) {
	e := echo.New()

	// Initialize wordbank
	wb, err := wordbank.NewWithPG(options.DBURI)
	if err != nil {
		return nil, err
	}

	// Initialize controllers
	wordCtrl := &word.WordController{
		WB: wb,
	}
	wordCtrl.Routes(e)

	return &Server{
		Addr:  options.Addr,
		DBURI: options.DBURI,
		e:     e,
	}, nil
}

func (s *Server) Run() error {
	return s.e.Start(s.Addr)
}
