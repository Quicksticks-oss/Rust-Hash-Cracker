extern crate crypto_hash;
use crypto_hash::{digest, Algorithm};
use std::time::Instant;

fn main() {
    let crackme = "532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25".to_string();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!.".chars().collect::<Vec<_>>();
    let mut current = Vec::new();
    let base = alphabet.len();

    let start_time = Instant::now();

    for len in 1..=12 {
        let result = generate_combinations(&alphabet, &mut current, len, base, &crackme);
        if result {
            break;
        }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed Time: {:?}", elapsed_time);
}

fn generate_combinations(
    alphabet: &Vec<char>,
    current: &mut Vec<char>,
    len: usize,
    base: usize,
    hash_base: &String,
) -> bool {
    if len == 0 {
        let string = current.iter().collect::<String>();
        let hash = digest(Algorithm::SHA256, string.as_bytes());
        let hash_hex = hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        if hash_hex == *hash_base {
            println!("Cracked: {}", string);
            return true;
        }
    } else {
        for i in 0..base {
            current.push(alphabet[i]);
            let result = generate_combinations(alphabet, current, len - 1, base, hash_base);
            if result {
                return true;
            }
            current.pop();
        }
    }
    false
}
