use std::env;
use std::fs::{read_dir, File};
use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use threadpool::ThreadPool;
use threadpool_scope::scope_with;

const NUM_JOBS: usize = 1024;
const NUM_CPU_CORES: usize = 10;
fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let res = count_all_sub_files_threaded(&args[1], &args[2], NUM_JOBS);

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

fn count_lines_file(path: &str) -> usize {
    let mut f = File::open(path).unwrap();
    let mut buf = String::from("value");
    let _ = f.read_to_string(&mut buf).unwrap();
    count_lines(&buf)
}

fn gather_all_sub_path(path: &str, suffix: &str, res: &mut Vec<String>) {
    read_dir(path).unwrap().for_each(|f| {
        let f = f.unwrap();
        let file_type = f.file_type().unwrap();
        let file_name = format!("{}/{}", path, f.file_name().to_str().unwrap());
        if file_type.is_dir() {
            gather_all_sub_path(&file_name, suffix, res);
        } else if file_type.is_file() {
            if file_name.ends_with(suffix) {
                res.push(file_name);
            }
        }
    });
}

fn split_immut_vec<T>(v: &Vec<T>, num_slices: usize) -> Vec<Box<&[T]>> {
    let num_slices = std::cmp::min(v.len(), num_slices);
    let slice_len = (v.len() + num_slices - 1) / num_slices;
    let mut res = vec![];
    let mut remaining = &v[..];
    while remaining.len() > 0 {
        let (head, tail) = remaining.split_at(std::cmp::min(slice_len, remaining.len()));
        remaining = tail;
        res.push(Box::new(head));
    }
    res
}

fn count_all_sub_files_threaded(path: &str, suffix: &str, num_slices: usize) -> usize {
    let mut v = vec![];
    gather_all_sub_path(path, suffix, &mut v);
    let v = split_immut_vec(&v, num_slices);
    let pool = ThreadPool::new(NUM_CPU_CORES);
    let res = AtomicUsize::new(0);
    scope_with(&pool, |s| {
        v.iter().for_each(|fs| {
            s.execute(|| {
                res.fetch_add(
                    fs.iter().map(|f| count_lines_file(f)).sum(),
                    Ordering::Relaxed,
                );
            })
        });
    });
    res.load(Ordering::Relaxed)
}
