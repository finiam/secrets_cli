mod decryption;
mod encryption;
fn main() {
    //encryption::encrypt_data("bla".to_string()).expect("failed");
    //println!("{:?}", decrypt());

    // https://secrets.finiam.com/Z1o3cFRUNWVtaGM9#HrQNjPREMOSQtfKfR2p5NUtFjZFhOHul

    // https://secrets.finiam.com/api/secrets/Z1o3cFRUNWVtaGM9

    let pass_phrase = encryption::generate_pass_phrase();
    let content = "Finiam".to_owned();

    let encrypted = encryption::encrypt_data(&content, &pass_phrase);

    println!("{:?}", content);
    println!("{:?}", pass_phrase);
    println!("{:?}", encrypted);
    println!("{:?}", decryption::decrypt_data(&encrypted, &pass_phrase));
}
