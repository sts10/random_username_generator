use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let file_input: Vec<String> = read_by_line("eff_large_wordlist.txt").unwrap();
    println!("First element from file is {:?}", file_input[0]);
    let mut eff_words: Vec<String> = vec![];
    for line in file_input {
        let this_word: String = line.split_whitespace().collect::<Vec<&str>>()[1].to_string();
        eff_words.push(this_word);
    }
    let first_word_of_username = eff_words.choose(&mut rand::thread_rng());
    let second_word_of_username = eff_words.choose(&mut rand::thread_rng());
    let rand_number: usize = rand::thread_rng().gen_range(0, 999);
    let rand_number_as_string: String = rand_number.to_string();

    println!(
        "Your random username is \n{}_{}{}",
        first_word_of_username.unwrap(),
        second_word_of_username.unwrap(),
        &rand_number_as_string
    );
}

fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                eprintln!("Error");
                continue;
            }
        }
    }
    Ok(vec)
}
