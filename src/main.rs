use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let eff_words = make_list("eff_large_wordlist.txt");
    println!("Some randomly generated usernames are:\n");
    for _count in 1..=10 {
        let rand_number_as_string: String = rand::thread_rng().gen_range(0, 999).to_string();

        println!(
            "{}{}{}{}",
            get_random_word(&eff_words),
            get_random_combiner(),
            get_random_word(&eff_words),
            &rand_number_as_string
        );
    }
}

fn make_list(file_path: &str) -> Vec<String> {
    let file_input: Vec<String> = match read_by_line(file_path) {
        Ok(r) => r,
        Err(e) => panic!("Error: {}", e),
    };
    let mut eff_words: Vec<String> = vec![];
    for line in file_input {
        let this_word: String = line.split_whitespace().collect::<Vec<&str>>()[1].to_string();
        eff_words.push(this_word);
    }
    eff_words
}

fn get_random_word(eff_words: &[String]) -> String {
    match eff_words.choose(&mut rand::thread_rng()) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random EFF word"),
    }
}

fn get_random_combiner() -> String {
    match vec!["", "_", "-"].choose(&mut rand::thread_rng()) {
        Some(combiner) => (*combiner).to_string(),
        None => " ".to_string(),
    }
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
