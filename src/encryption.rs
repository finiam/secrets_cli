//use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{aead::*, digest, pbkdf2, rand};
use std::num::NonZeroU32;
struct OneNonceSequence(Option<Nonce>);

impl OneNonceSequence {
    /// Constructs the sequence allowing `advance()` to be called
    /// `allowed_invocations` times.
    fn new(nonce: Nonce) -> Self {
        Self(Some(nonce))
    }
}

impl NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.0.take().ok_or(Unspecified)
    }
}
//use ring::{aead, digest, pbkdf2};
//use std::num::NonZeroU32;

//static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
//const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
//pub type Credential = [u8; CREDENTIAL_LEN];

//fn salt(username: &[u8], db_salt_component: [u8; 16]) -> Vec<u8> {
//    let mut salt = Vec::with_capacity(db_salt_component.len() + username.len());
//    salt.extend(db_salt_component.as_ref());
//    salt.extend(username);
//    salt
//}
//
//fn encrypt(blob: &[u8]) {
//    let db_salt_component = [
//        // This value was generated from a secure PRNG.
//        0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01,
//        0x8a,
//    ];
//    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
//
//    let salt = salt(blob, db_salt_component);
//    let pbkdf2_iterations = NonZeroU32::new(100_000).unwrap();
//    pbkdf2::derive(PBKDF2_ALG, pbkdf2_iterations, &salt, blob, &mut to_store);
//
//    let aead::Algorithm::SealingKey();
//}
//
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
fn derive_key(
    password: String,
) -> Result<([u8; CREDENTIAL_LEN], [u8; CREDENTIAL_LEN]), Unspecified> {
    //const salt = window.crypto.getRandomValues(new Uint8Array(16));
    //   const iv = window.crypto.getRandomValues(new Uint8Array(12));
    //   const passwordKey = await getPasswordKey(passphrase);
    //   const aesKey = await deriveKey(passwordKey, salt, ['encrypt']);
    //   const encryptedContent = await window.crypto.subtle.encrypt(
    //     {
    //       name: 'AES-GCM',
    //       iv: iv
    //     },
    //     aesKey,
    //     new TextEncoder().encode(plainText)
    //   );

    let mut salt = [0u8; CREDENTIAL_LEN];
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    let iterations = NonZeroU32::new(250000).unwrap();

    let rng = rand::SystemRandom::new();
    rng.fill(&mut salt)?;

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    println!("Salt: {:?}", salt);
    println!("PBKDF2 hash: {:?}", pbkdf2_hash);

    Ok((salt, pbkdf2_hash))
}

fn encrypt_data(plain_text: String) -> Result<(), Unspecified> {
    let password = "123ABCDE".to_string();

    let (salt, hash): ([u8; 32], [u8; 32]) = derive_key(password).unwrap();

    let content = plain_text.as_bytes().to_vec();

    let mut in_out = content.clone();
    for _ in 0..AES_256_GCM.tag_len() {
        in_out.push(0);
    }
    // Opening key used to decrypt data
    //NonceSequence.
    //let opening_key =
    //    OpeningKey::new(&AES_256_GCM, NonceSequence::assume_unique_for_key(&key).into()).unwrap();

    let mut nonce = [0; 12];

    // Fill nonce with random data
    let rand = rand::SystemRandom::new();
    rand.fill(&mut nonce).unwrap();
    //iv === nonce
    // Sealing key used to encrypt data
    let key = UnboundKey::new(&AES_256_GCM, &hash).unwrap();
    let key2 = UnboundKey::new(&AES_256_GCM, &hash).unwrap();
    let nonce_unique = Nonce::assume_unique_for_key(nonce);
    let nonce_unique2 = Nonce::assume_unique_for_key(nonce);
    let nonce_sequence = OneNonceSequence::new(nonce_unique);
    let nonce_sequence2 = OneNonceSequence::new(nonce_unique2);
    let mut sealing_key: SealingKey<OneNonceSequence> = BoundKey::new(key, nonce_sequence);
    let mut opening_key: OpeningKey<OneNonceSequence> = BoundKey::new(key2, nonce_sequence2);

    //let sealing_key = SealingKey::new(&AES_256_GCM, &key).unwrap();
    //
    sealing_key
        .seal_in_place_append_tag(Aad::from(vec![1, 2, 3]), &mut in_out)
        .unwrap();

    let decrypted_data = opening_key
        .open_in_place(Aad::from(vec![1, 2, 3]), &mut in_out)
        .unwrap();
    println!("{:?}", String::from_utf8(decrypted_data.to_vec()).unwrap());

    // Random data must be used only once per encryption

    // Encrypt data into in_out variable
    //    let output_size = seal_in_place(
    //        &sealing_key,
    //        &nonce,
    //        &additional_data,
    //        &mut in_out,
    //        AES_256_GCM.tag_len(),
    //    )
    //    .unwrap();

    //let should_succeed = pbkdf2::verify(
    //    pbkdf2::PBKDF2_HMAC_SHA512,
    //    n_iter,
    //    &salt,
    //    password.as_bytes(),
    //    &pbkdf2_hash,
    //);
    Ok(())
}
