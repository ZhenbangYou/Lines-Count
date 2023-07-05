use std::env;
use std::fs::{read_dir, File};
use std::io::Read;
use string_builder::Builder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = count_all_sub_files(&args[1], &args[2]);
    println!("{res}")
}

fn remove_comments_and_space(src: &str) -> String {
    enum State {
        Code,
        SingleLineComment,
        MultiLineComment,
        Slash,
        Asterisk,
    }
    let mut state = State::Code;
    let mut builder = Builder::new(0);
    for c in src.chars().into_iter() {
        match state {
            State::Code => {
                if c != '/' {
                    const SPACE_SET: [char; 3] = [' ', '\t', '\r'];
                    if !SPACE_SET.contains(&c) {
                        builder.append(c);
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
                    builder.append('/');
                    builder.append(c);
                    state = State::Code;
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
    builder.string().unwrap()
}

fn count_lines_str(src: &str) -> usize {
    let src = remove_comments_and_space(src);
    let src = src.trim_start();
    fn count_double_endl(src: &str) -> usize {
        let mut res = 0;
        for i in 0..src.len() - 1 {
            if &src[i..i + 2] == "\n\n" {
                res += 1;
            }
        }
        res
    }
    src.chars().filter(|c| *c == '\n').count() - count_double_endl(&src)
}

fn count_lines_file(path: &str) -> usize {
    let mut f = File::open(path).unwrap();
    let mut buf = String::from("value");
    let _ = f.read_to_string(&mut buf).unwrap();
    count_lines_str(&buf)
}

fn count_all_sub_files(path: &str, suffix: &str) -> usize {
    read_dir(path)
        .unwrap()
        .map(|f| {
            let f = f.unwrap();
            let file_type = f.file_type().unwrap();
            let file_name = format!("{}/{}", path, f.file_name().into_string().unwrap());
            if file_type.is_dir() {
                count_all_sub_files(&file_name, suffix)
            } else if file_type.is_file() {
                if file_name.ends_with(suffix) {
                    println!("{file_name}");
                    count_lines_file(&file_name)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .fold(0, |a, b| a + b)
}
