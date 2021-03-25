package asserthttp

import (
	"bytes"
	"io/ioutil"
	"net/http"
	"testing"

	"github.com/andybalholm/crlf"
	"github.com/bradleyjkemp/cupaloy"
	"github.com/stretchr/testify/assert"
)

func AssertResponse(t *testing.T, response *http.Response) {
	t.Helper()

	var buf bytes.Buffer

	err := response.Write(&buf)
	assert.NoError(t, err)

	bytes, err := ioutil.ReadAll(crlf.NewReader(&buf))
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, bytes)
}
