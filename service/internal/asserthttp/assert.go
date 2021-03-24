package asserthttp

import (
	"bytes"
	"net/http"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/stretchr/testify/assert"
)

func AssertResponse(t *testing.T, response *http.Response) {
	t.Helper()

	var buf bytes.Buffer

	err := response.Write(&buf)
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, buf)
}
