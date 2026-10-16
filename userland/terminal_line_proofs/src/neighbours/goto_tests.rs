// NONOS Operating System (AGPL-3.0-or-later)
//! Go to line. An editor that lands one line from where it was told is worse
//! than one that cannot jump, so the boundaries are what matter here.

mod offsets;
mod parsing;

const DOC: &[u8] = b"one\ntwo\nthree\nfour";

fn line_at(buf: &[u8], off: usize) -> String {
    let end = buf[off..].iter().position(|&b| b == b'\n').map(|p| off + p).unwrap_or(buf.len());
    String::from_utf8(buf[off..end].to_vec()).unwrap()
}
