use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use base64:{decode};
                                      //use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

//fn derive_key(password: String) -> String {
//    let salt = SaltString::generate(&mut OsRng);
//
//    // Hash password to PHC string ($pbkdf2-sha256$...)
//    let password_hash = Pbkdf2
//        .hash_password(&password.as_bytes(), &salt)
//        .unwrap()
//        .to_string();
//
//    // Verify password against PHC string
//
//    password_hash
//}
//
//pub fn encrypt() -> String {
//    let password = "123ABCDE".to_string();
//
//    let hash: String = derive_key(password);
//
//    let key = Key::from_slice(b"an example very very secret key.");
//    let cipher = Aes256Gcm::new(key);
//
//    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//
//    let ciphertext = cipher
//        .encrypt(nonce, b"plaintext message".as_ref())
//        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
//
//    println!("{:?}", ciphertext);
//    String::from_utf8(ciphertext).unwrap()
//}

pub fn decrypt() -> String {
    let base64 = "mu8BdsoEZLLv3DGuPLOHNnxAH4ouHeNG/lrOAINzIOG6hO8DSuTGU4V3uICaG00=";
    let password = "AY5bRWdFUJpUmyPGtmdKuB85qVgMNQvD";

    let text = decode(base64).unwrap();


    let salt = text[0..16];
    let iv = text[16..28];
    let data = text[28..];

    let aesKey = deriveKey(password, salt);



    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
}
