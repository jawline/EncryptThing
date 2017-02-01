use openssl::crypto::symm::{encrypt, decrypt};
use openssl::crypto::symm::Type::{AES_128_CBC};

pub fn decrypt_message(data: &Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	decrypt(AES_128_CBC, key, iv, data)
}

pub fn encrypt_message(data: &Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	encrypt(AES_128_CBC, key, iv, data)
}