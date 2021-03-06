use failure::{Error,ResultExt, Fail};

use ring::digest;
use serde_derive::{Deserialize, Serialize};

pub fn rand_str(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.sample_iter(&Alphanumeric).take(len).collect()
}

#[inline(always)]
pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().simple().to_string()
}

pub fn sha512(input: &[u8]) -> Vec<u8> {
    let digest = digest::digest(&digest::SHA512, input);
    (&digest.as_ref()).to_vec()
}

pub fn argon2_hash(password: &[u8], secret: &[u8]) -> Result<String, Error> {
    use argonautica::Hasher;
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(secret)
        .hash()
        .context("Encryption failed")?;

    Ok(hash)
}

pub fn argon2_verify(password: &[u8], secret: &[u8], hash: &str) -> Result<bool, Error> {
    use argonautica::Verifier;

    let mut verifier = Verifier::default();
    let valid = verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret)
        .verify()
        .context("Verification failed")?;

    Ok(valid)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum CryptoError {
    // A plain enum with no data in any of its variants
    //
    // For example:
    #[fail(display = "Error encrypting")]
    Encryption,
    // ...
    #[fail(display = "Error while validating")]
    Validation,
}
