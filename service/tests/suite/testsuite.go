package suite

import (
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/rs/zerolog/pkgerrors"
	"github.com/sazzer/newlanding/service/internal/service"
	"github.com/stretchr/testify/suite"
)

// Test Suite to manage interactions with the New Landing service.
type TestSuite struct {
	suite.Suite
	service *service.Service
}

// Setup the entire test suite.
func (s *TestSuite) SetupSuite() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	zerolog.SetGlobalLevel(zerolog.DebugLevel)
	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack

	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: time.RFC3339}).With().Caller().Logger()
}

// Setup the next test to run.
func (s *TestSuite) SetupTest() {
	service := service.New(service.Config{
		HTTP: service.HTTPConfig{
			Port: 0,
		},
	})

	s.service = &service
}

// Tear down the last test that was run.
func (s *TestSuite) TearDownTest() {
	s.service = nil
}

func (s *TestSuite) Request(req *http.Request) *http.Response {
	w := httptest.NewRecorder()

	s.service.ServeHTTP(w, req)

	return w.Result()
}

// Expose the testify suite.Run since we're sharing the same package name.
func Run(t *testing.T, s suite.TestingSuite) {
	t.Helper()

	suite.Run(t, s)
}
