use crate::error::IntuneError;
use kanidm_hsm_crypto::{LoadableIdentityKey, LoadableMsOapxbcRsaKey};
use openssl::x509::X509;

pub struct IntuneClient {}

impl IntuneClient {
    pub fn enroll_device(
        &self,
        _transport_key: &LoadableMsOapxbcRsaKey,
        _cert_key: &LoadableIdentityKey,
    ) -> Result<X509, IntuneError> {
        Err(IntuneError::NotImplemented)
    }
}
