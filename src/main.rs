use rayon::prelude::*;
use std::env;
use std::fs::{read_dir, File};
use std::io::Read;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let args: Vec<_> = env::args().collect();
    let res = count_all_sub_files_threaded(&args[1], &args[2..]);

    println!("{} ms", now.elapsed().as_millis());

    println!("{res}")
}

fn count_lines(src: &String) -> usize {
    enum State {
        Code,
        SingleLineComment,
        MultiLineComment,
        Slash,
        Asterisk,
    }
    let mut state = State::Code;
    let mut is_prev_endl = true;
    let mut res = 0;
    for c in src.chars().into_iter() {
        match state {
            State::Code => {
                if c != '/' {
                    const SPACE_SET: [char; 3] = [' ', '\t', '\r'];
                    if !SPACE_SET.contains(&c) {
                        if c == '\n' && !is_prev_endl {
                            res += 1;
                        }
                        is_prev_endl = c == '\n';
                    }
                } else {
                    state = State::Slash;
                }
            }
            State::SingleLineComment => {
                if c == '\n' {
                    state = State::Code;
                }
            }
            State::MultiLineComment => {
                if c == '*' {
                    state = State::Asterisk;
                }
            }
            State::Slash => {
                if c == '/' {
                    state = State::SingleLineComment;
                } else if c == '*' {
                    state = State::MultiLineComment;
                } else {
                    state = State::Code;
                    if c == '\n' {
                        res += 1;
                    }
                    is_prev_endl = c == '\n';
                }
            }
            State::Asterisk => {
                if c == '/' {
                    state = State::Code;
                } else if c != '*' {
                    state = State::MultiLineComment;
                }
            }
        }
    }
    res
}

fn count_lines_file(path: &String) -> usize {
    let mut buf = String::new();
    match File::open(path).unwrap().read_to_string(&mut buf) {
        Ok(_) => count_lines(&buf),
        Err(e) => {
            eprintln!("error when reading `{path}`: {e}");
            0
        }
    }
}

fn gather_all_sub_path(path: &String, suffixes: &[String], res: &mut Vec<String>) {
    read_dir(path).unwrap().for_each(|f| {
        let f = f.unwrap();
        let file_type = f.file_type().unwrap();
        let file_name = format!("{}/{}", path, f.file_name().to_str().unwrap());
        if file_type.is_dir() {
            gather_all_sub_path(&file_name, suffixes, res);
        } else if file_type.is_file() {
            if suffixes.iter().any(|s| file_name.ends_with(s)) {
                res.push(file_name);
            }
        }
    })
}

fn count_all_sub_files_threaded(path: &String, suffixes: &[String]) -> usize {
    let mut v = vec![];
    gather_all_sub_path(path, suffixes, &mut v);
    v.par_iter().map(count_lines_file).sum()
}
