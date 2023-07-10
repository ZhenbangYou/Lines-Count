use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use walkdir::WalkDir;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let res = count_all_sub_files_threaded(&args[1], &args[2..]);

    println!("{} ms", now.elapsed().as_millis());

    println!("{res}");
}

fn count_lines(src: &str) -> usize {
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
    let mut f = File::open(path).unwrap();
    let mut buf = String::from("value");
    match f.read_to_string(&mut buf) {
        Ok(_) => count_lines(&buf),
        Err(e) => {
            eprintln!("error when reading `{path}`: {e}");
            0
        }
    }
}

fn gather_all_sub_path(path: &String, suffixes: &[String]) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|p| !p.path_is_symlink())
        .map(|x| String::from(x.path().to_str().unwrap()))
        .filter(|f| suffixes.iter().any(|s| f.ends_with(s)))
        .collect()
}

fn count_all_sub_files_threaded(path: &String, suffixes: &[String]) -> usize {
    gather_all_sub_path(path, suffixes)
        .par_iter()
        .map(count_lines_file)
        .sum()
}
