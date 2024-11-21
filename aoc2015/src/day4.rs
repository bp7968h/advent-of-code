use md5::{Digest, Md5};
use std::sync::{Arc, Mutex};
use std::thread;

const THREAD_COUNT: usize = 8;

pub fn run(secret: &'static str, num_zeros: &'static str) -> Option<usize> {
    let found = Arc::new(Mutex::new(None));
    let mut handles = vec![];

    for thread_id in 0..THREAD_COUNT {
        let found_clone = Arc::clone(&found);
        let handle = thread::spawn(move || {
            for i in (thread_id..).step_by(THREAD_COUNT) {
                // Check if another thread already found a solution
                if found_clone.lock().unwrap().is_some() {
                    break;
                }

                let sec_num = format!("{}{}", secret, i);
                let mut hasher = Md5::new();
                hasher.update(sec_num.as_bytes());
                let digest = hasher.finalize();
                let digest_hex = format!("{:x}", digest);

                if digest_hex.starts_with(num_zeros) {
                    // Store the result in the shared state
                    let mut result = found_clone.lock().unwrap();
                    if result.is_none() {
                        *result = Some(i);
                    }
                    break;
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the result
    match *found.lock().unwrap() {
        Some(i) => println!("The lowest number is: {}", i),
        None => println!("No number found."),
    };

    let mut locked_value = found.lock().unwrap();
    locked_value.take()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_hash_abcdef609043() {
        let secret: &'static str = "abcdef";
        assert_eq!(run(secret, "00000"), Some(609043));
    }

    #[test]
    fn for_hash_pqrstuv1048970() {
        let secret: &'static str = "pqrstuv";
        assert_eq!(run(secret, "00000"), Some(1048970));
    }
}