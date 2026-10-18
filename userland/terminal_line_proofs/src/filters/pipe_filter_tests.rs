// NONOS Operating System (AGPL-3.0-or-later)
//! The pipe filters take the flags `help` gives them. In a pipe `head -n 3`
//! printed ten lines, `wc -w` counted lines and `grep -c` took `-c` for the
//! pattern; `cut -d , -f 2` with spaces read neither value.

mod arguments;
mod counting;

fn lines(v: &[&str]) -> Vec<Vec<u8>> {
    v.iter().map(|s| s.as_bytes().to_vec()).collect()
}

fn show(v: Vec<Vec<u8>>) -> Vec<String> {
    v.into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}

fn ten() -> Vec<Vec<u8>> {
    (1..=12).map(|i| format!("l{i}").into_bytes()).collect()
}
