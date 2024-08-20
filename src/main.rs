
use std::fs;
use std::io;
use std::process::exit;

fn main() -> io::Result<()> {
    let path = homedir::my_home()
        .unwrap()
        .unwrap()
        .join("words.txt");
    let Ok(words) = fs::read_to_string(&path) else {
        println!("words.txt not found");
        exit(0);
    };
    let words = words
        .split_whitespace()
        .filter_map(|word| {
            if word.trim() == "" {
                None
            } else {
                Some(word.trim().to_string())
            }
        })
        .collect::<Vec<_>>();

    if words.len() == 0 {
        println!("words.txt does not contain any words to choose from");
        exit(0);
    }

    let len = words.len();
    let index = (rand::random::<u16>() as usize) % len;
    println!("{}", words[index]);
    let words = words
        .into_iter()
        .enumerate()
        .filter_map(|(i, word)| {
            if i == index {
                None
            } else {
                Some(word)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let Ok(()) = fs::write(path, words) else {
        println!("failed to remove the chosen word from words.txt");
        exit(0);
    };

    Ok(())
}
