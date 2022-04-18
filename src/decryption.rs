use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, NewAead, Nonce};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Params, Pbkdf2,
};

fn derive_key(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let salt = SaltString::b64_encode(salt).unwrap();

    let params = Params {
        rounds: 250000,
        output_length: 32,
    };

    let password_hash = Pbkdf2
        .hash_password_customized(password, None, None, params, &salt)
        .expect("Failed to hash passowrd");

    password_hash.hash.expect("No hash").as_bytes().to_vec()
}

fn decrypt(data: &[u8], iv: &[u8], aes_key: &[u8]) -> String {
    let payload = Payload::from(data);

    // if you want to create cipher with key
    //let key = Key::from_slice(aes_key);
    //let cipher = Aes256Gcm::new(key);
    let cipher =
        Aes256Gcm::new_from_slice(aes_key).expect("Failed to create cipher with provided aes key");

    let nonce = Nonce::from_slice(iv);

    let decrypt_bytes = cipher
        .decrypt(nonce, payload)
        .expect("failed to decrypt bytes");

    String::from_utf8(decrypt_bytes).expect("Failed to convert bytes to plaintext")
}

pub fn decrypt_data(encrypted_text: &str, pass_phrase: &str) -> String {
    let encrypted_data_buffer: Vec<u8> = base64::decode(encrypted_text).unwrap();

    let salt = &encrypted_data_buffer[0..16];
    let iv = &encrypted_data_buffer[16..28];
    let data = &encrypted_data_buffer[28..];
    let password_key = pass_phrase.as_bytes();

    let aes_key = derive_key(password_key, salt);

    decrypt(data, iv, &aes_key)
}

#[cfg(test)]
mod test {
    use crate::decryption::decrypt_data;

    #[test]
    fn decrypt_data_test() {
        assert_eq!(
            decrypt_data(
                "wJDT3o+k+kdjWh7Dq5xn8QjvKgE96AbKswMxr+d9dpVuiukmsxcILq3EbWAj4LGu2RNkog==",
                "3EP1sqZP2G6bA8tmpVWlDSCyy4uPb1Cm"
            ),
            "Teste123".to_owned()
        );
    }
}
