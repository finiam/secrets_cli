//mod encryption2;
mod decryption;
fn main() {
    //encryption::encrypt_data("bla".to_string()).expect("failed");
    //println!("{:?}", decrypt());

    // https://secrets.finiam.com/Z1o3cFRUNWVtaGM9#HrQNjPREMOSQtfKfR2p5NUtFjZFhOHul

    // https://secrets.finiam.com/api/secrets/Z1o3cFRUNWVtaGM9

    println!(
        "{:?}",
        decryption::decrypt_data(
            "wJDT3o+k+kdjWh7Dq5xn8QjvKgE96AbKswMxr+d9dpVuiukmsxcILq3EbWAj4LGu2RNkog==",
            "3EP1sqZP2G6bA8tmpVWlDSCyy4uPb1Cm"
        )
    );
}
