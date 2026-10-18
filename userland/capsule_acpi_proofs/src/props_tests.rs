// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The PROPERTIES wire: a tree with every kind of value and nested nodes
//! reads back as written, and every limit and malformation refuses the
//! whole tree, in the writer and the reader alike.

use crate::props::wire::*;
use crate::props::wire_read::decode;
use crate::props::wire_write::encode;

fn prop(name: &str, val: Val) -> Prop {
    Prop { name: name.as_bytes().to_vec(), val }
}

fn camera() -> Tree {
    let gpio = Val::Pkg(vec![Val::Pkg(vec![Val::Ref(b"\\_SB.GPO0".to_vec()), Val::Int(3)])]);
    let ep = Tree {
        name: b"endpoint@0".to_vec(),
        props: vec![prop("data-lanes", Val::Pkg(vec![Val::Int(1), Val::Int(2)]))],
        children: vec![],
    };
    let port = Tree {
        name: b"port@0".to_vec(),
        props: vec![prop("reg", Val::Int(0))],
        children: vec![ep],
    };
    let props = vec![
        prop("rotation", Val::Int(180)),
        prop("compatible", Val::Str(b"sony,imx219".to_vec())),
        prop("reset-gpios", gpio),
    ];
    Tree { name: vec![], props, children: vec![port] }
}

fn nest(levels: u32) -> Tree {
    (0..levels).fold(Tree::default(), |t, _| Tree { children: vec![t], ..Tree::default() })
}

fn pkg(depth: u32) -> Val {
    (0..depth).fold(Val::Int(1), |v, _| Val::Pkg(vec![v]))
}

#[test]
fn a_tree_reads_back_as_written() {
    let t = camera();
    let b = encode(&t).expect("encodes");
    assert_eq!(decode(&b), Some(t));
    assert_eq!(decode(&encode(&Tree::default()).unwrap()), Some(Tree::default()), "no _DSD");
}

#[test]
fn limits_refuse_the_tree() {
    assert!(encode(&nest(MAX_LEVELS - 1)).is_some());
    assert_eq!(encode(&nest(MAX_LEVELS)), None, "too many levels of nodes");
    let deep = |d| Tree { props: vec![prop("p", pkg(d))], ..Tree::default() };
    assert!(encode(&deep(MAX_DEPTH)).is_some());
    assert_eq!(encode(&deep(MAX_DEPTH + 1)), None, "packages too deep");
    let many = Tree { props: vec![prop("p", Val::Int(0)); MAX + 1], ..Tree::default() };
    assert_eq!(encode(&many), None);
    let long = Tree { name: vec![b'n'; 256], ..Tree::default() };
    assert_eq!(encode(&long), None, "a name past 255 bytes");
    let big = Tree { props: vec![prop("s", Val::Str(vec![b'x'; 300])); MAX], ..Tree::default() };
    assert_eq!(encode(&big), None, "past MAX_WIRE");
}

#[test]
fn malformed_bytes_are_refused() {
    let b = encode(&camera()).unwrap();
    for cut in 0..b.len() {
        assert_eq!(decode(&b[..cut]), None, "cut at {cut}");
    }
    assert_eq!(decode(&[b.clone(), vec![0]].concat()), None, "trailing byte");
    assert_eq!(decode(&[0, 1, 1, b'p', 9, 0]), None, "unknown kind");
    assert_eq!(decode(&[0, (MAX + 1) as u8]), None, "count past MAX");
    let mut deep = vec![0, 1, 1, b'p'];
    deep.extend(std::iter::repeat_n([4, 1], MAX_DEPTH as usize + 1).flatten());
    deep.extend([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(decode(&deep), None, "packages too deep");
}
