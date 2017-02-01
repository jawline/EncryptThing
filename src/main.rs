extern crate rustc_serialize;
extern crate openssl;
extern crate rand;

mod lssl;

use rustc_serialize::base64::FromBase64;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::STANDARD;

use rand::os::OsRng;
use rand::Rng;

pub fn generate_key(length: usize) -> Vec<u8> {
    let mut rng = OsRng::new().unwrap();
    (0..length).map(|_| rng.gen::<u8>()).collect()
}

fn decrypt(key: &str, ciphertext: &str, iv: &Vec<u8>) {
    let ciphertext = ciphertext.from_base64().unwrap();
    let key = key.from_base64().unwrap();
    let decrypted_bytes = lssl::decrypt_message(&ciphertext, &iv, &key);
    let bytes_as_text = String::from_utf8(decrypted_bytes).unwrap();
    println!("{}", bytes_as_text);
}

fn encrypt(key: &str, message: &str, iv: &Vec<u8>) {
    let message = message.as_bytes().to_vec();
    let key = key.from_base64().unwrap();
    let encrypted_bytes = lssl::encrypt_message(&message, &iv, &key);
    println!("{}", encrypted_bytes.to_base64(STANDARD));
}

fn main() {
    let iv: Vec<u8> = vec![0; 128];
	match std::env::args().nth(1) {
        Some(ref x) if x == "decrypt" => {
            decrypt(&std::env::args().nth(2).unwrap(), &std::env::args().nth(3).unwrap(), &iv);
        },
        Some(ref x) if x == "encrypt" => {
            encrypt(&std::env::args().nth(2).unwrap(), &std::env::args().nth(3).unwrap(), &iv);
        },
        Some(ref x) if x == "generate_key" => {
            println!("{}", generate_key(16).to_base64(STANDARD));
        },
		Some(ref x) => {
			println!("Enter generate, encrypt or decrypt {}", x);
		}
		_ => { println!("Incorrect number of arguments"); }
	}
}
