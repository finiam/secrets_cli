use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, NewAead, Nonce};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Params, Pbkdf2,
};
use rand::{self, Rng};

const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const PASSWORD_LEN: usize = 32;

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

fn encrypt(data: &[u8], iv: &[u8], aes_key: &[u8]) -> Vec<u8> {
    let payload = Payload::from(data);

    // if you want to create cipher with key
    //let key = Key::from_slice(aes_key);
    //let cipher = Aes256Gcm::new(key);
    let cipher =
        Aes256Gcm::new_from_slice(aes_key).expect("Failed to create cipher with provided aes key");

    let nonce = Nonce::from_slice(iv);

    let encrypt_bytes = cipher
        .encrypt(nonce, payload)
        .expect("failed to decrypt bytes");

    encrypt_bytes
}

pub fn generate_pass_phrase() -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

pub fn encrypt_data(plain_text: &str, pass_phrase: &str) -> String {
    let salt = rand::thread_rng().gen::<[u8; 16]>().to_vec();
    let iv = rand::thread_rng().gen::<[u8; 12]>().to_vec();

    let password_key = pass_phrase.as_bytes();

    let aes_key = derive_key(password_key, &salt);

    let data = plain_text.as_bytes();

    let output = encrypt(data, &iv, &aes_key);
    let output_bytes = output.to_vec();

    let content_buffer = [salt, iv, output_bytes].concat();

    base64::encode(content_buffer)
}

#[cfg(test)]
mod test {
    use crate::decryption::decrypt_data;
    use crate::encryption::{encrypt_data, generate_pass_phrase};

    #[test]
    fn encrypt_data_test() {
        let pass_phrase = generate_pass_phrase();
        let content = "Finiam".to_owned();

        let encrypted = encrypt_data(&content, &pass_phrase);

        assert_eq!(decrypt_data(&encrypted, &pass_phrase), content);
    }
}
