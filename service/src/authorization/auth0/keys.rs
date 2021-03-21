use std::{cell::RefCell, sync::Mutex};

use super::Domain;
use biscuit::jwk::{JWKSet, JWK};
use reqwest::{Client, StatusCode};

/// Wrapper around the JWK Keys, allowing us to automatically fetch them when needed.
pub struct Keys {
    /// The Auth0 Domain to get the keys from.
    domain: Domain,
    /// The HTTP Client to use to get the keys.
    client: Client,
    /// The cached JWK keys
    cache: Mutex<RefCell<JWKSet<()>>>,
}

impl Keys {
    /// Create a new wrapper around the keys.
    ///
    /// # Parameters
    /// - `domain` - The Auth0 Domain
    pub fn new(domain: Domain) -> Self {
        let client = Client::new();
        let keys = JWKSet { keys: vec![] };

        Self {
            domain,
            client,
            cache: Mutex::new(RefCell::new(keys)),
        }
    }

    /// Get the key that matches the requested Algorithm and Key ID.
    ///
    /// This will use a key from the cache if a matching one is present, or else retrieve the latest
    /// set of keys from Auth0 and cache those first.
    ///
    /// If the requested Key can not be found after fetching from Auth0 then an error is returned.
    ///
    /// # Parameters
    /// - `kid` - The ID of the key to retrieve.
    ///
    /// # Returns
    /// The matching key, if one could be found.
    #[tracing::instrument(skip(self))]
    pub async fn get(&self, kid: &str) -> Option<JWK<()>> {
        let lock = self.cache.lock().unwrap();
        let mut entry = lock.borrow_mut();

        if entry.find(kid).is_none() {
            tracing::debug!(kid = ?kid, "Requested key not present in cache");
            if let Some(keys) = self.fetch().await {
                entry.keys = keys.keys;
            }
        }

        let key = entry.find(kid);

        tracing::debug!(kid = ?kid, key = ?key, "Found key");

        key.cloned()
    }

    /// Fetch the keys from Auth0 and store them into the cache.
    ///
    /// # Returns
    /// The keyset retrieved from Auth0.
    #[tracing::instrument(skip(self))]
    async fn fetch(&self) -> Option<JWKSet<()>> {
        let result = self
            .client
            .get(&self.domain.build_url("/.well-known/jwks.json"))
            .send()
            .await;
        tracing::debug!(result = ?result, "JWKS result");

        let result = match result {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    Some(r)
                } else {
                    tracing::error!(response = ?r, "Failed to request JWKS");

                    let body = r.text().await;
                    tracing::error!(response_body = ?body, "Response body from failure");

                    None
                }
            }
            Err(e) => {
                tracing::error!(e = ?e, "Failed to request JWKS");
                None
            }
        }?;

        let body: JWKSet<()> = match result.json().await {
            Ok(b) => Some(b),
            Err(e) => {
                tracing::error!(e = ?e, "Failed to parse JWKS response");
                None
            }
        }?;
        tracing::debug!(result = ?body, "JWKS body");

        Some(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use biscuit::jwa;
    use mockito::mock;

    #[actix_rt::test]
    async fn get_key_success() {
        let _ = env_logger::try_init();

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "keys": [
                  {
                      "alg": "RS256",
                      "e": "AQAB",
                      "kid": "myKeyId",
                      "kty": "RSA",
                      "n": "uNaM8aXf0OJSmjc1iBvJEKdB9LFNn-UfZI7mZURSDhCFNxNb8jwP7z6d_DsAbEfNbE4yQ3eZ86qT6speuVB2n5wGqXA0-rKEgYTEA_2isE88EwFoo04_284dvRbpSpeWmIn45_vM-RQKZE_tBqkm00k6eGO_llW5knLMiXcQ_AhNfdHiNcszY3rI_Xc-6uJFvwXnxy61AZbRp8gvvWzkNpnbzeCu40EnNMp6FpAIREdyQkrKaMPfS1Mlg_S0QhhUiT7NionT-nzbl5d2hlsO5_33S838NL5_T7Ts6-3viH0WLIJKAyC6KoF5zxONuztIetyZ_JkErflPAQOtm5TcCQ",
                      "use": "sig",
                      "x5c": [
                          "MIIDETCCAfmgAwIBAgIJJtgqNutHq0anMA0GCSqGSIb3DQEBCwUAMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTAeFw0yMTAzMDYxMTIxMThaFw0zNDExMTMxMTIxMThaMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALjWjPGl39DiUpo3NYgbyRCnQfSxTZ/lH2SO5mVEUg4QhTcTW/I8D+8+nfw7AGxHzWxOMkN3mfOqk+rKXrlQdp+cBqlwNPqyhIGExAP9orBPPBMBaKNOP9vOHb0W6UqXlpiJ+Of7zPkUCmRP7QapJtNJOnhjv5ZVuZJyzIl3EPwITX3R4jXLM2N6yP13PuriRb8F58cutQGW0afIL71s5DaZ283gruNBJzTKehaQCERHckJKymjD30tTJYP0tEIYVIk+zYqJ0/p825eXdoZbDuf990vN/DS+f0+07Ovt74h9FiyCSgMguiqBec8Tjbs7SHrcmfyZBK35TwEDrZuU3AkCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUduuktYhD46z4ToVvLoqrjrusadEwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAQkI/lwZuKLCzMv5oxPo7KzIgNQOdQrMGjrN/vnVMmdpFnN3cgF4hpTgEbCfnjMUGfGujVqJ69ZEG4/sL7bSJD2YOkvS982KTfFG8TsWwEXRBeInES7FiXkm/bbs4tX5JCAFBHCtfaSCHSK93cg+at/SPDjDFiONFH17UyJmIQi2e3S2tUYTK6/scZzNIy2T5ZcMjBC3VExojQduJaN+Y5YMClTuxIofOSrduyMT7bNwBaHvC3B4f6s/2yUvRd+50BCEixbC1etxZ3ordwbBAAs8yxETbpVEsYJVTSwoCQz6i8dlZ0HQmurJh9ezTrWdmkl/WZPLDWwSKJ1eC7aRct"
                      ],
                      "x5t": "ceXBgISC99AL6JII5KhC__fuEP4"
                  }
              ]
          }"#)
            .create();

        let sut = Keys::new(Domain::new(mockito::server_url()));
        let key = sut.get("myKeyId").await;

        let_assert!(Some(key) = key);

        check!(key.common.key_id.unwrap() == "myKeyId");
        check!(
            key.common.algorithm.unwrap()
                == jwa::Algorithm::Signature(jwa::SignatureAlgorithm::RS256)
        );

        let_assert!(biscuit::jwk::AlgorithmParameters::RSA(_) = key.algorithm);

        m.assert();
    }

    #[actix_rt::test]
    async fn get_unknown_key() {
        let _ = env_logger::try_init();

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"keys": []}"#)
            .create();

        let sut = Keys::new(Domain::new(mockito::server_url()));
        let key = sut.get("myKeyId").await;

        check!(key.is_none());

        m.assert();
    }

    #[actix_rt::test]
    async fn get_key_failed() {
        let _ = env_logger::try_init();

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body(r#"Unknown host"#)
            .create();

        let sut = Keys::new(Domain::new(mockito::server_url()));
        let key = sut.get("myKeyId").await;

        check!(key.is_none());

        m.assert();
    }

    #[actix_rt::test]
    async fn get_key_from_cache() {
        let _ = env_logger::try_init();

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "keys": [
                  {
                      "alg": "RS256",
                      "e": "AQAB",
                      "kid": "myKeyId",
                      "kty": "RSA",
                      "n": "uNaM8aXf0OJSmjc1iBvJEKdB9LFNn-UfZI7mZURSDhCFNxNb8jwP7z6d_DsAbEfNbE4yQ3eZ86qT6speuVB2n5wGqXA0-rKEgYTEA_2isE88EwFoo04_284dvRbpSpeWmIn45_vM-RQKZE_tBqkm00k6eGO_llW5knLMiXcQ_AhNfdHiNcszY3rI_Xc-6uJFvwXnxy61AZbRp8gvvWzkNpnbzeCu40EnNMp6FpAIREdyQkrKaMPfS1Mlg_S0QhhUiT7NionT-nzbl5d2hlsO5_33S838NL5_T7Ts6-3viH0WLIJKAyC6KoF5zxONuztIetyZ_JkErflPAQOtm5TcCQ",
                      "use": "sig",
                      "x5c": [
                          "MIIDETCCAfmgAwIBAgIJJtgqNutHq0anMA0GCSqGSIb3DQEBCwUAMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTAeFw0yMTAzMDYxMTIxMThaFw0zNDExMTMxMTIxMThaMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALjWjPGl39DiUpo3NYgbyRCnQfSxTZ/lH2SO5mVEUg4QhTcTW/I8D+8+nfw7AGxHzWxOMkN3mfOqk+rKXrlQdp+cBqlwNPqyhIGExAP9orBPPBMBaKNOP9vOHb0W6UqXlpiJ+Of7zPkUCmRP7QapJtNJOnhjv5ZVuZJyzIl3EPwITX3R4jXLM2N6yP13PuriRb8F58cutQGW0afIL71s5DaZ283gruNBJzTKehaQCERHckJKymjD30tTJYP0tEIYVIk+zYqJ0/p825eXdoZbDuf990vN/DS+f0+07Ovt74h9FiyCSgMguiqBec8Tjbs7SHrcmfyZBK35TwEDrZuU3AkCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUduuktYhD46z4ToVvLoqrjrusadEwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAQkI/lwZuKLCzMv5oxPo7KzIgNQOdQrMGjrN/vnVMmdpFnN3cgF4hpTgEbCfnjMUGfGujVqJ69ZEG4/sL7bSJD2YOkvS982KTfFG8TsWwEXRBeInES7FiXkm/bbs4tX5JCAFBHCtfaSCHSK93cg+at/SPDjDFiONFH17UyJmIQi2e3S2tUYTK6/scZzNIy2T5ZcMjBC3VExojQduJaN+Y5YMClTuxIofOSrduyMT7bNwBaHvC3B4f6s/2yUvRd+50BCEixbC1etxZ3ordwbBAAs8yxETbpVEsYJVTSwoCQz6i8dlZ0HQmurJh9ezTrWdmkl/WZPLDWwSKJ1eC7aRct"
                      ],
                      "x5t": "ceXBgISC99AL6JII5KhC__fuEP4"
                  }
              ]
          }"#)
            .create();

        let sut = Keys::new(Domain::new(mockito::server_url()));

        let key = sut.get("myKeyId").await;
        let_assert!(Some(key) = key);

        m.assert();

        let key2 = sut.get("myKeyId").await;
        let_assert!(Some(key2) = key2);

        check!(key == key2);
    }

    #[actix_rt::test]
    async fn get_key_from_cache_miss() {
        let _ = env_logger::try_init();

        let m1 = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "keys": [
                  {
                      "alg": "RS256",
                      "e": "AQAB",
                      "kid": "myKeyId1",
                      "kty": "RSA",
                      "n": "uNaM8aXf0OJSmjc1iBvJEKdB9LFNn-UfZI7mZURSDhCFNxNb8jwP7z6d_DsAbEfNbE4yQ3eZ86qT6speuVB2n5wGqXA0-rKEgYTEA_2isE88EwFoo04_284dvRbpSpeWmIn45_vM-RQKZE_tBqkm00k6eGO_llW5knLMiXcQ_AhNfdHiNcszY3rI_Xc-6uJFvwXnxy61AZbRp8gvvWzkNpnbzeCu40EnNMp6FpAIREdyQkrKaMPfS1Mlg_S0QhhUiT7NionT-nzbl5d2hlsO5_33S838NL5_T7Ts6-3viH0WLIJKAyC6KoF5zxONuztIetyZ_JkErflPAQOtm5TcCQ",
                      "use": "sig",
                      "x5c": [
                          "MIIDETCCAfmgAwIBAgIJJtgqNutHq0anMA0GCSqGSIb3DQEBCwUAMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTAeFw0yMTAzMDYxMTIxMThaFw0zNDExMTMxMTIxMThaMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALjWjPGl39DiUpo3NYgbyRCnQfSxTZ/lH2SO5mVEUg4QhTcTW/I8D+8+nfw7AGxHzWxOMkN3mfOqk+rKXrlQdp+cBqlwNPqyhIGExAP9orBPPBMBaKNOP9vOHb0W6UqXlpiJ+Of7zPkUCmRP7QapJtNJOnhjv5ZVuZJyzIl3EPwITX3R4jXLM2N6yP13PuriRb8F58cutQGW0afIL71s5DaZ283gruNBJzTKehaQCERHckJKymjD30tTJYP0tEIYVIk+zYqJ0/p825eXdoZbDuf990vN/DS+f0+07Ovt74h9FiyCSgMguiqBec8Tjbs7SHrcmfyZBK35TwEDrZuU3AkCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUduuktYhD46z4ToVvLoqrjrusadEwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAQkI/lwZuKLCzMv5oxPo7KzIgNQOdQrMGjrN/vnVMmdpFnN3cgF4hpTgEbCfnjMUGfGujVqJ69ZEG4/sL7bSJD2YOkvS982KTfFG8TsWwEXRBeInES7FiXkm/bbs4tX5JCAFBHCtfaSCHSK93cg+at/SPDjDFiONFH17UyJmIQi2e3S2tUYTK6/scZzNIy2T5ZcMjBC3VExojQduJaN+Y5YMClTuxIofOSrduyMT7bNwBaHvC3B4f6s/2yUvRd+50BCEixbC1etxZ3ordwbBAAs8yxETbpVEsYJVTSwoCQz6i8dlZ0HQmurJh9ezTrWdmkl/WZPLDWwSKJ1eC7aRct"
                      ],
                      "x5t": "ceXBgISC99AL6JII5KhC__fuEP4"
                  }
              ]
          }"#)
            .create();

        let m2 = mock("GET", "/.well-known/jwks.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
          "keys": [
              {
                  "alg": "RS256",
                  "e": "AQAB",
                  "kid": "myKeyId2",
                  "kty": "RSA",
                  "n": "uNaM8aXf0OJSmjc1iBvJEKdB9LFNn-UfZI7mZURSDhCFNxNb8jwP7z6d_DsAbEfNbE4yQ3eZ86qT6speuVB2n5wGqXA0-rKEgYTEA_2isE88EwFoo04_284dvRbpSpeWmIn45_vM-RQKZE_tBqkm00k6eGO_llW5knLMiXcQ_AhNfdHiNcszY3rI_Xc-6uJFvwXnxy61AZbRp8gvvWzkNpnbzeCu40EnNMp6FpAIREdyQkrKaMPfS1Mlg_S0QhhUiT7NionT-nzbl5d2hlsO5_33S838NL5_T7Ts6-3viH0WLIJKAyC6KoF5zxONuztIetyZ_JkErflPAQOtm5TcCQ",
                  "use": "sig",
                  "x5c": [
                      "MIIDETCCAfmgAwIBAgIJJtgqNutHq0anMA0GCSqGSIb3DQEBCwUAMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTAeFw0yMTAzMDYxMTIxMThaFw0zNDExMTMxMTIxMThaMCYxJDAiBgNVBAMTG2Rldi1uZXdsYW5kaW5nLmV1LmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALjWjPGl39DiUpo3NYgbyRCnQfSxTZ/lH2SO5mVEUg4QhTcTW/I8D+8+nfw7AGxHzWxOMkN3mfOqk+rKXrlQdp+cBqlwNPqyhIGExAP9orBPPBMBaKNOP9vOHb0W6UqXlpiJ+Of7zPkUCmRP7QapJtNJOnhjv5ZVuZJyzIl3EPwITX3R4jXLM2N6yP13PuriRb8F58cutQGW0afIL71s5DaZ283gruNBJzTKehaQCERHckJKymjD30tTJYP0tEIYVIk+zYqJ0/p825eXdoZbDuf990vN/DS+f0+07Ovt74h9FiyCSgMguiqBec8Tjbs7SHrcmfyZBK35TwEDrZuU3AkCAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUduuktYhD46z4ToVvLoqrjrusadEwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAQkI/lwZuKLCzMv5oxPo7KzIgNQOdQrMGjrN/vnVMmdpFnN3cgF4hpTgEbCfnjMUGfGujVqJ69ZEG4/sL7bSJD2YOkvS982KTfFG8TsWwEXRBeInES7FiXkm/bbs4tX5JCAFBHCtfaSCHSK93cg+at/SPDjDFiONFH17UyJmIQi2e3S2tUYTK6/scZzNIy2T5ZcMjBC3VExojQduJaN+Y5YMClTuxIofOSrduyMT7bNwBaHvC3B4f6s/2yUvRd+50BCEixbC1etxZ3ordwbBAAs8yxETbpVEsYJVTSwoCQz6i8dlZ0HQmurJh9ezTrWdmkl/WZPLDWwSKJ1eC7aRct"
                  ],
                  "x5t": "ceXBgISC99AL6JII5KhC__fuEP4"
              }
          ]
      }"#)
        .create();

        let sut = Keys::new(Domain::new(mockito::server_url()));

        let key = sut.get("myKeyId1").await;
        let_assert!(Some(key) = key);

        m1.assert();

        let key2 = sut.get("myKeyId2").await;
        let_assert!(Some(key2) = key2);

        check!(key != key2);

        m2.assert();
    }
}
