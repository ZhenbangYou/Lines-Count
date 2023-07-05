use string_builder::Builder;

fn main() {
    println!("Hello, world!");
}

fn remove_space(s: &str) -> String {
    const SPACE_SET: [char; 3] = [' ', '\t', '\r'];
    s.chars().filter(|c| !SPACE_SET.contains(c)).collect()
}

fn count_occur(pattern: &str, s: &str) -> usize {
    s.matches(pattern).count()
}

fn remove_comments(src: &str) -> String {
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
                    builder.append(c);
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

fn countLinesStr(src: &str) -> usize {
    let src = remove_space(&remove_comments(src));
    count_occur("\n", &src) - count_occur("\n", &src)
}
