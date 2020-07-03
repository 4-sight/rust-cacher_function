use std::io;
use std::thread;
use std::time::Duration;
use std::time::Instant;

extern crate caching_function;
use caching_function::Cache;

fn main() {
    println!("\nCaching function\n");

    let mut exit = false;
    let mut cached = Cache::new(|num: u8| -> u32 {
        thread::sleep(Duration::from_secs(2));
        num as u32 * 100
    });

    while !exit {
        println!("Please enter a number between 1 & 10:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        match input {
            "exit" => exit = true,
            &_ => match input.parse::<u8>() {
                Ok(n) => {
                    if n < 1 {
                        println!("That number's too small!\n");
                        continue;
                    } else if n > 10 {
                        println!("That number's too large!\n");
                        continue;
                    } else {
                        let start_time = Instant::now();

                        println!(
                            "Result: {}\nTime to fetch result: {}s\n",
                            cached.value(n),
                            start_time.elapsed().as_secs()
                        )
                    }
                }
                Err(_) => {
                    println!("That's not a number!\n");
                    continue;
                }
            },
        }
    }
}
