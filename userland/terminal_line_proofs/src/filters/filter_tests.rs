// NONOS Operating System (AGPL-3.0-or-later)
//! The pipeline text filters.

mod reverse;
mod sort_grep;
mod uniq;

fn lines(v: &[&str]) -> Vec<Vec<u8>> {
    v.iter().map(|s| s.as_bytes().to_vec()).collect()
}

fn show(v: Vec<Vec<u8>>) -> Vec<String> {
    v.into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}
