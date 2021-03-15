package auth0_test

import (
	"context"
	"testing"

	"github.com/sazzer/newlanding/service/internal/users/auth0"
	"github.com/stretchr/testify/assert"
	"gopkg.in/h2non/gock.v1"
)

func TestGetKnownKey(t *testing.T) {
	t.Parallel()

	defer gock.Off()
	// nolint:lll
	mock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).BodyString(`{
				"keys": [
						{
								"alg": "RS256",
								"e": "AQAB",
								"kid": "dctbOWUQuxjs4TD5dhHio",
								"kty": "RSA",
								"n": "uNaM8aXf0OJSmjc1iBvJEKdB9LFNn-UfZI7mZURSDhCFNxNb8jwP7z6d_DsAbEfNbE4yQ3eZ86qT6speuVB2n5wGqXA0-rKEgYTEA_2isE88EwFoo04_284dvRbpSpeWmIn45_vM-RQKZE_tBqkm00k6eGO_llW5knLMiXcQ_AhNfdHiNcszY3rI_Xc-6uJFvwXnxy61AZbRp8gvvWzkNpnbzeCu40EnNMp6FpAIREdyQkrKaMPfS1Mlg_S0QhhUiT7NionT-nzbl5d2hlsO5_33S838NL5_T7Ts6-3viH0WLIJKAyC6KoF5zxONuztIetyZ_JkErflPAQOtm5TcCQ",
								"use": "sig",
								"x5c": [
										"MIIDETCCAfmgAwIBAgIJJtgqNutHq0anMA0GCSqGSIb3DQEBCwUAMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTAeFw0yMTAzMDYxMTIxMThaFw0zNDExMTMxMTIxMThaMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALjWjPGl39DiUpo3NYgbyRCnQfSxTZ/lH2SO5mVEUg4QhTcTW/I8D+8+nfw7AGxHzWxOMkN3mfOqk+rKXrlQdp+cBqlwNPqyhIGExAP9orBPPBMBaKNOP9vOHb0W6UqXlpiJ+Of7zPkUCmRP7QapJtNJOnhjv5ZVuZJyzIl3EPwITX3R4jXLM2N6yP13PuriRb8F58cutQGW0afIL71s5DaZ283gruNBJzTKehaQCERHckJKymjD30tTJYP0tEIYVIk+zYqJ0/p825eXdoZbDuf990vN/DS+f0+07Ovt74h9FiyCSgMguiqBec8Tjbs7SHrcmfyZBK35TwEDrZuU3AkCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUduuktYhD46z4ToVvLoqrjrusadEwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAQkI/lwZuKLCzMv5oxPo7KzIgNQOdQrMGjrN/vnVMmdpFnN3cgF4hpTgEbCfnjMUGfGujVqJ69ZEG4/sL7bSJD2YOkvS982KTfFG8TsWwEXRBeInES7FiXkm/bbs4tX5JCAFBHCtfaSCHSK93cg+at/SPDjDFiONFH17UyJmIQi2e3S2tUYTK6/scZzNIy2T5ZcMjBC3VExojQduJaN+Y5YMClTuxIofOSrduyMT7bNwBaHvC3B4f6s/2yUvRd+50BCEixbC1etxZ3ordwbBAAs8yxETbpVEsYJVTSwoCQz6i8dlZ0HQmurJh9ezTrWdmkl/WZPLDWwSKJ1eC7aRct"
								],
								"x5t": "ceXBgISC99AL6JII5KhC__fuEP4"
						}
				]
		}`)

	sut := auth0.NewKeyset("https://example.xx.auth0.com")

	keys, err := sut.FetchKeys(context.Background())
	assert.NoError(t, err)

	assert.Equal(t, 1, keys.Len())

	key, ok := keys.LookupKeyID("dctbOWUQuxjs4TD5dhHio")
	assert.True(t, ok)
	assert.Equal(t, "RS256", key.Algorithm())

	assert.True(t, mock.Done())
}

func TestGetUnknownKey(t *testing.T) {
	t.Parallel()

	defer gock.Off()
	mock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(404)

	sut := auth0.NewKeyset("https://example.xx.auth0.com")

	_, err := sut.FetchKeys(context.Background())
	assert.Equal(t, auth0.ErrFetchKeys, err)

	assert.True(t, mock.Done())
}
