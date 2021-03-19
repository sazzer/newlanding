package hal_test

import (
	"encoding/json"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/newlanding/service/internal/response/hal"
	"github.com/stretchr/testify/assert"
)

type Empty struct {
	hal.Document
}

type Full struct {
	hal.Document
	Name    string   `json:"name"`
	Age     uint     `json:"age"`
	Colours []string `json:"colours"`
}

func TestRenderEmptyModel(t *testing.T) {
	t.Parallel()

	model := Empty{}

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}

func TestRenderSingleLink(t *testing.T) {
	t.Parallel()

	model := Empty{}
	model.WithLink("self", hal.NewLink("/"))

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}

func TestRenderRepeaedLinks(t *testing.T) {
	t.Parallel()

	model := Empty{}
	model.WithLink("item", hal.NewTemplateLink("/item/{id}", map[string]interface{}{"id": 1}))
	model.WithLink("item", hal.NewTemplateLink("/item/{id}", map[string]interface{}{"id": 2}))
	model.WithLink("item", hal.NewTemplateLink("/item/{id}", map[string]interface{}{"id": 3}))

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}

func TestRenderMixedLinks(t *testing.T) {
	t.Parallel()

	model := Empty{}
	model.WithLink("item", hal.NewLink("/item/1"))
	model.WithLink("item", hal.NewLink("/item/2"))
	model.WithLink("self", hal.NewLink("/"))

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}

func TestRenderNamedLinks(t *testing.T) {
	t.Parallel()

	model := Empty{}
	model.WithLink("item", hal.NewNamedTemplateLink("one", "/item/{id}", map[string]interface{}{"id": 1}))
	model.WithLink("item", hal.NewNamedTemplateLink("two", "/item/{id}", map[string]interface{}{"id": 2}))
	model.WithLink("item", hal.NewNamedTemplateLink("three", "/item/{id}", map[string]interface{}{"id": 3}))

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}

func TestRenderWithData(t *testing.T) {
	t.Parallel()

	model := Full{
		Name:    "Graham",
		Age:     38,
		Colours: []string{"red", "green", "blue"},
	}
	model.WithLink("colour", hal.NewNamedLink("red", "/colours/red"))
	model.WithLink("colour", hal.NewNamedLink("green", "/colours/green"))
	model.WithLink("colour", hal.NewNamedLink("blue", "/colours/blue"))
	model.WithLink("self", hal.NewLink("/"))

	rendered, err := json.MarshalIndent(model, "", "  ")
	assert.NoError(t, err)

	cupaloy.SnapshotT(t, rendered)
}
