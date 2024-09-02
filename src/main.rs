mod read;
mod error;
use rand::*;
use error::ProgramErrorKind;
use read::read_stdin;
use std::{env, fs, io::Write, path};

#[derive(Debug, PartialEq)]
enum ProgramMode {
    GENERATE(String),
    MUTATE(String),
}

fn get_args(min: usize) -> Result<Vec<String>, ProgramErrorKind> {
    let mut result: Vec<String> = env::args().collect();
    result.remove(0);
    if result.len() < min {
        return Err(ProgramErrorKind::NotEnoughtArgs);
    }
    return Ok(result);
}

fn parse_args(arg_arr: Vec<String>) -> Result<ProgramMode, ProgramErrorKind> {
    let mode: ProgramMode;
    match &arg_arr[0][..] {
        "gen" | "generate" => {
            mode = ProgramMode::GENERATE(arg_arr[1].clone());
        }
        "mut" | "mutate" => {
            mode = ProgramMode::MUTATE(arg_arr[1].clone());
        }
        _ => {
            return Err(ProgramErrorKind::Invalid);
        }
    }
    return Ok(mode);
}

fn gen_rand_char(mut rng_obj: rand::rngs::ThreadRng, mode: usize) -> Result<char, ()> {
    let a: String = "aiueo".to_string();
    let b: String = "bcdfghjklmnpqrstvwxyz".to_string();
    let mut result: char = '0';
    let mut random_num: usize;

    if mode == 0 {
        random_num = rng_obj.gen_range(0..b.len());
        result = b.chars().nth(random_num).unwrap_or('0');
    }

    if mode == 1 {
        random_num = rng_obj.gen_range(0..a.len());
        result = a.chars().nth(random_num).unwrap_or('0');
    }

    if result != '0' {
        return Ok(result);
    }

    return Err(());
}

fn gen_rand_word(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut rng_obj = rand::thread_rng();
    let a: String = "aiueo".to_string();
    let b: String = "bcdfghjklmnpqrstvwxyz".to_string();
    for _ in 0..n {
        let mut tmp_str: String = String::new();
        let word_len: i32 = rng_obj.gen_range(2..5);
        for _ in 0..word_len {
            let rand: i32 = rng_obj.gen_range(0..3);
            // b a
            // b a b
            // a b
            // b
            match rand {
                0 => {
                    let c: char = b.chars().nth(rng_obj.gen_range(0..b.len())).unwrap();
                    let d: char = a.chars().nth(rng_obj.gen_range(0..a.len())).unwrap();
                    tmp_str.push_str(&format!("{}{}", c, d));
                }
                1 => {
                    let c: char = b.chars().nth(rng_obj.gen_range(0..b.len())).unwrap();
                    let d: char = a.chars().nth(rng_obj.gen_range(0..a.len())).unwrap();
                    let e: char = b.chars().nth(rng_obj.gen_range(0..b.len())).unwrap();
                    tmp_str.push_str(&format!("{}{}{}", c, d, e));
                }
                2 => {
                    let c: char = a.chars().nth(rng_obj.gen_range(0..a.len())).unwrap();
                    let d: char = b.chars().nth(rng_obj.gen_range(0..b.len())).unwrap();
                    tmp_str.push_str(&format!("{}{}", c, d));
                }
                3 => {
                    let c: char = b.chars().nth(rng_obj.gen_range(0..b.len())).unwrap();
                    tmp_str.push_str(&format!("{}", c));
                }
                _ => {}
            }
        }
        result.push(tmp_str);
    }
    return result;
}

fn read_file(s_path: &str) -> Result<String, ProgramErrorKind> {
    let file_content: String = match fs::read_to_string(s_path) {
        Ok(val) => val,
        Err(_) => return Err(ProgramErrorKind::FileFailedR),
    };
    return Ok(file_content);
}

fn write_file(s_path: &str, content: &str) -> Result<(), ProgramErrorKind> {
    if path::Path::new(s_path).exists() {
        let mut file = match fs::OpenOptions::new().append(true).open(s_path) {
            Ok(val) => val,
            Err(_) => return Err(ProgramErrorKind::FileFailedW),
        };
        writeln!(file, "{}", content).unwrap();
        return Ok(());
    }
    let _ = match fs::File::create(s_path) {
        Ok(_) => write_file(s_path, content),
        Err(_) => return Err(ProgramErrorKind::FileCreationFail),
    };
    return Ok(());
}

fn is_vowel(c: char) -> bool {
    let a: String = "aiueo".to_string();
    if a.find(c).is_some() {
        return true;
    }
    return false;
}

fn convert_str_to_i32(str_in: &str, min: i32) -> Result<i32, ()> {
    let convert: i32 = str_in.trim().parse().unwrap_or(-1);
    if convert < min {
        return Err(());
    }
    return Ok(convert);
}

fn main() {
    let prog: Result<Vec<String>, ProgramErrorKind> = get_args(2);
    let mut file_path: String = String::new();
    match prog {
        Ok(_) => {}
        Err(err) => {
            err.print_and_exit();
        }
    };
    let mode: Result<ProgramMode, ProgramErrorKind> = parse_args(prog.unwrap());
    match mode {
        Ok(ref a) => match a {
            ProgramMode::GENERATE(b) => file_path = b.to_string(),
            ProgramMode::MUTATE(b) => file_path = b.to_string(),
        },
        Err(err) => {
            err.print_and_exit();
        }
    };

    let mut final_word: Vec<String> = Vec::new();
    let mut random_word: Vec<String> = Vec::new();

    if mode.unwrap() == ProgramMode::MUTATE(file_path.clone()) {
        let result: Result<String, ProgramErrorKind> = read_file(&file_path);
        match result {
            Ok(_) => {}
            Err(err) => err.print_and_exit(),
        };
        let res_str = result.unwrap();
        let result_arr: Vec<&str> = res_str.split('\n').collect();
        let mut rng_obj = rand::thread_rng();
        for i in 0..result_arr.len() {
            let mut current_str: String = String::new();
            let current_char: Vec<char> = result_arr[i].chars().collect();
            for j in 0..current_char.len() {
                let random_num = rng_obj.gen_range(0..2);
                let c = current_char[j];
                match random_num {
                    0 => {
                        if is_vowel(c) {
                            current_str.push(gen_rand_char(rng_obj.clone(), 1).unwrap_or(c));
                        } else {
                            current_str.push(gen_rand_char(rng_obj.clone(), 0).unwrap_or(c));
                        }
                    }
                    1 => current_str.push(c),
                    2 => current_str.push(c),
                    _ => {}
                }
            }
            final_word.push(current_str.clone());
        }
        match final_word.pop() {
            Some(val) => val,
            None => {
                eprintln!("Empty file.");
                let exit: ProgramErrorKind = ProgramErrorKind::FileEmpty;
                exit.print_err();
                std::process::exit(exit as i32 + 1);
            }
        };
        println!("{:?}", final_word);
        std::process::exit(0);
    }

    println!("INFO: Type `q` or `quit` or `exit` to exit.");
    println!("INFO: Type `1..n` to generate `n` amount.");
    println!("INFO: Type `s`n`` to save the `n` index.");
    'gen_loop: loop {
        let mut something: String = read_stdin("Input : ");
        something = something.trim_end().to_string();
        match &something[..] {
            "q" | "quit" | "exit" => break 'gen_loop,
            "p" | "print" => println!("{:?}", final_word),
            _ => {
                match convert_str_to_i32(&something, 1) {
                    Ok(val) => {
                        random_word.clear();
                        let generated_word: Vec<String> = gen_rand_word(val);
                        for i in 0..generated_word.len() {
                            let word: String = generated_word[i].clone();
                            random_word.push(word.clone());
                            println!("[{}]{}", i, word);
                        }
                    }
                    Err(()) => {}
                }

                let something_len = something.len();
                let random_word_len = random_word.len();

                if something.chars().nth(0) == Some('s')
                    && something_len > 1
                    && random_word_len >= 1
                {
                    let num: &str = &something[1..something.len()];
                    match convert_str_to_i32(num, 0) {
                        Ok(val) => {
                            if val > random_word_len as i32 - 1 {
                                continue;
                            }
                            final_word.push(random_word[val as usize].clone());
                        }
                        Err(()) => {}
                    }
                }
            }
        }
    }
    for i in 0..final_word.len() {
        let res = write_file(&file_path, &final_word[i]);
        match res {
            Ok(_) => {}
            Err(err) => err.print_and_exit(),
        }
    }
}
