package home_test

import (
	"net/http/httptest"
	"testing"

	"github.com/sazzer/newlanding/service/internal/asserthttp"
	"github.com/sazzer/newlanding/service/tests/suite"
)

type HomeSuite struct {
	suite.TestSuite
}

func (s HomeSuite) TestHomeDocumentUnauthenticated() {
	req := httptest.NewRequest("GET", "/", nil)

	res := s.Request(req)

	asserthttp.AssertResponse(s.T(), res)
}

func TestHomeSuite(t *testing.T) {
	t.Parallel()

	suite.Run(t, new(HomeSuite))
}
