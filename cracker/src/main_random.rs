extern crate rand;
extern crate crypto_hash;

use rand::Rng;
use crypto_hash::{digest, Algorithm};
use std::time::Instant;

fn main() {

    let crackme = "532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25";

    // Define the characters that can be used in the random string.
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // Define the length of the random string you want to generate.
    let length = 4;
    // Measure the time it takes to calculate the SHA-256 hash.
    let start_time = Instant::now();

    loop {
        // Generate the random string.
        let random_string: String = (0..length)
            .map(|_| {
                let idx = rand::thread_rng().gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();

        let hash = digest(Algorithm::SHA256, random_string.as_bytes());

        // Convert the hash to a hexadecimal string.
        let hash_hex = hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        if hash_hex == crackme{
            println!("Cracked: {}", random_string);
            println!("SHA-256 Hash: {}", hash_hex);
            break;
        }
    }

    let end_time = Instant::now();
    // Calculate and print the elapsed time.
    let elapsed_time = end_time - start_time;
    println!("Elapsed Time: {:?}", elapsed_time);
}