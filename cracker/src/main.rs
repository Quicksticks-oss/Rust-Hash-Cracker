extern crate crypto_hash;
use crypto_hash::{digest, Algorithm};
use std::time::Instant;

fn main() {
    let crackme = "532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25".to_string();

    let charset =  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!.".chars().collect::<Vec<_>>(); // Define your custom character set here
    let mut counter = 0;

    let max_length = 12;

    let start_time = Instant::now();

    loop {
        let string_representation = counter_to_string(counter, &charset);
        let hash = digest(Algorithm::SHA256, string_representation.as_bytes());
        let hash_hex = hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        //println!("{}", string_representation);
        if hash_hex == crackme {
            println!("Cracked: {}", string_representation);
            break;
        }
        // Increment the counter
        counter += 1;
        if string_representation.len() > max_length {
            break;
        }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed Time: {:?}", elapsed_time);
}

fn counter_to_string(mut n: u64, charset: &[char]) -> String {
    let mut result = String::new();
    let charset_len = charset.len();

    while n > 0 {
        let remainder = (n % charset_len as u64) as usize;
        result.push(charset[remainder]);

        n /= charset_len as u64;
    }

    result.chars().rev().collect()
}