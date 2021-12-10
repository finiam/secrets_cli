use ring::{aead, digest, pbkdf2};
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn salt(username: &[u8], db_salt_component: [u8; 16]) -> Vec<u8> {
    let mut salt = Vec::with_capacity(db_salt_component.len() + username.len());
    salt.extend(db_salt_component.as_ref());
    salt.extend(username);
    salt
}

fn encrypt(blob: &[u8]) {
    let db_salt_component = [
        // This value was generated from a secure PRNG.
        0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01,
        0x8a,
    ];
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];

    let salt = salt(blob, db_salt_component);
    let pbkdf2_iterations = NonZeroU32::new(100_000).unwrap();
    pbkdf2::derive(PBKDF2_ALG, pbkdf2_iterations, &salt, blob, &mut to_store);

    let aead::Algorithm::SealingKey();
}
