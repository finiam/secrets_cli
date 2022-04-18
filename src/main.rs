mod decryption;
mod encryption;
mod file_utils;
mod http_lib;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(author, version = "0.1.0", about = "Secrets command line")]
struct Cli {
    #[clap(short, long)]
    content: Option<String>,

    #[clap(short, long, default_value = "2100")]
    expiry: u32,

    #[clap(short, long)]
    file: Option<String>,

    #[clap(short, long)]
    read: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if args.content == None && args.file == None && args.read == None {
        println!("No content to read or secret to open");
        std::process::exit(1);
    }

    if args.content != None && args.file != None {
        println!("Define only one: content or file");
        std::process::exit(1);
    }

    let content: Option<String> = if let Some(filename) = args.file {
        Some(file_utils::read_file_to_string(&filename)?)
    } else {
        args.content
    };

    let api = http_lib::APIClient::new();
    match content {
        Some(content) => {
            let expiry = args.expiry;

            let pass_phrase = encryption::generate_pass_phrase();

            let encrypted = encryption::encrypt_data(&content, &pass_phrase);

            let room_id = api.create_secret(encrypted, expiry).await?;

            println!("https://secrets.finiam.com/{}#{}", room_id, pass_phrase);

            Ok(())
        }
        None => {
            if let Some(url) = args.read {
                let regex = Regex::new(r"^(https://)?secrets.finiam.com/(.*)?#(.*)?$")?;

                let captures = regex.captures(&url);

                let cap = match captures {
                    Some(cap) => cap,
                    None => {
                        println!("Not a recognizable url");
                        std::process::exit(1);
                    }
                };

                let room_id = cap.get(2).expect("Not a recognizable url").as_str();

                let pass_phrase = cap.get(3).expect("Not a recognizable url").as_str();

                if api.check_if_room_exists(room_id).await? {
                    let content = api.get_room_secret(room_id).await?;
                    let decrypt = decryption::decrypt_data(&content, pass_phrase);

                    println!("{}", decrypt);

                    Ok(())
                } else {
                    println!("Invalid or expired secret");
                    std::process::exit(1);
                }
            } else {
                println!("No arguments to process");
                std::process::exit(1);
            }
        }
    }
}
