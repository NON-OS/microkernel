// NONOS Operating System (AGPL-3.0-or-later)
//! Drawing a directory tree, checked against the shapes that break one.

mod counts;
mod shape;

use crate::tree_render::render;

fn paths(list: &[&str]) -> Vec<Vec<u8>> {
    list.iter().map(|s| s.as_bytes().to_vec()).collect()
}

fn drawn(list: &[&str]) -> Vec<String> {
    render(&paths(list)).into_iter().map(|r| String::from_utf8(r.line).unwrap()).collect()
}
