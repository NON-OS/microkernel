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

//! Reading a line's statements into its control flow. Each statement either
//! is a simple command or starts with a keyword: `if`, `then`, `else`,
//! `fi`, `for`, `while`, `do`, `done`. A keyword may carry a command
//! after it (`then echo yes`); `for` carries its name and words. A block
//! left open, a keyword out of place, a block nested past `MAX_DEPTH` or a
//! background `&` is refused before anything runs.

use crate::statements::{split_program, Conn};
use alloc::vec::Vec;

/// How deep blocks may nest.
pub const MAX_DEPTH: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kw {
    If,
    Then,
    Else,
    Fi,
    For,
    While,
    Do,
    Done,
}

enum Item<'a> {
    Kw(Conn, Kw, &'a [u8]),
    Cmd(Conn, &'a [u8]),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Cmd(&'a [u8]),
    If { cond: Vec<(Conn, Node<'a>)>, then: Vec<(Conn, Node<'a>)>, els: Vec<(Conn, Node<'a>)> },
    For { name: &'a [u8], words: &'a [u8], body: Vec<(Conn, Node<'a>)> },
    While { cond: Vec<(Conn, Node<'a>)>, body: Vec<(Conn, Node<'a>)> },
}

/// A line read into its blocks.
#[derive(Debug, PartialEq, Eq)]
pub struct Script<'a>(pub Vec<(Conn, Node<'a>)>);

pub fn parse(line: &[u8]) -> Result<Script<'_>, &'static [u8]> {
    let items = items(line)?;
    let mut at = 0;
    let top = block(&items, &mut at, &[], 0)?;
    match at == items.len() {
        true => Ok(Script(top)),
        false => Err(b"syntax: a keyword out of place"),
    }
}

fn items(line: &[u8]) -> Result<Vec<Item<'_>>, &'static [u8]> {
    let mut out = Vec::new();
    for st in split_program(line) {
        if st.background {
            return Err(b"background jobs are not available in this shell");
        }
        statement(&mut out, st.conn, st.body);
    }
    Ok(out)
}

/// One statement's items. After \`then\`, \`else\` or \`do\` the rest is a
/// statement of its own, which may open a block in turn (\`do for ...\`).
fn statement<'a>(out: &mut Vec<Item<'a>>, conn: Conn, body: &'a [u8]) {
    let (word, rest) = first_word(body);
    match keyword(word) {
        Some(k @ (Kw::Then | Kw::Else | Kw::Do)) => {
            out.push(Item::Kw(conn, k, &[]));
            if !rest.is_empty() {
                statement(out, Conn::Always, rest);
            }
        }
        Some(k) => out.push(Item::Kw(conn, k, rest)),
        None if !body.is_empty() => out.push(Item::Cmd(conn, body)),
        None => {}
    }
}

fn keyword(w: &[u8]) -> Option<Kw> {
    Some(match w {
        b"if" => Kw::If,
        b"then" => Kw::Then,
        b"else" => Kw::Else,
        b"fi" => Kw::Fi,
        b"for" => Kw::For,
        b"while" => Kw::While,
        b"do" => Kw::Do,
        b"done" => Kw::Done,
        _ => return None,
    })
}

fn first_word(b: &[u8]) -> (&[u8], &[u8]) {
    let b = trim(b);
    match b.iter().position(|c| *c == b' ' || *c == b'\t') {
        Some(i) => (&b[..i], trim(&b[i + 1..])),
        None => (b, &[]),
    }
}

fn trim(b: &[u8]) -> &[u8] {
    let ws = |c: &u8| matches!(c, b' ' | b'\t');
    let s = b.iter().position(|c| !ws(c)).unwrap_or(b.len());
    let e = b.iter().rposition(|c| !ws(c)).map_or(s, |i| i + 1);
    &b[s..e]
}

type Block<'a> = Vec<(Conn, Node<'a>)>;

/// Read nodes until one of `until` (not consumed) or the end.
fn block<'a>(
    items: &[Item<'a>],
    at: &mut usize,
    until: &[Kw],
    depth: usize,
) -> Result<Block<'a>, &'static [u8]> {
    if depth > MAX_DEPTH {
        return Err(b"syntax: blocks nested too deep");
    }
    let mut out = Vec::new();
    while let Some(item) = items.get(*at) {
        match *item {
            Item::Cmd(c, body) => {
                *at += 1;
                out.push((c, Node::Cmd(body)));
            }
            Item::Kw(_, k, _) if until.contains(&k) => return Ok(out),
            Item::Kw(c, k, rest) => {
                *at += 1;
                out.push((c, compound(items, at, k, rest, depth)?));
            }
        }
    }
    match until.is_empty() {
        true => Ok(out),
        false => Err(b"syntax: a block is not closed"),
    }
}

fn compound<'a>(
    items: &[Item<'a>],
    at: &mut usize,
    k: Kw,
    rest: &'a [u8],
    depth: usize,
) -> Result<Node<'a>, &'static [u8]> {
    let d = depth + 1;
    match k {
        Kw::If => {
            let cond = opened(items, at, rest, &[Kw::Then], d)?;
            let then = body_of(items, at, Kw::Then, &[Kw::Else, Kw::Fi], d)?;
            let els = match next_kw(items, *at) {
                Some(Kw::Else) => body_of(items, at, Kw::Else, &[Kw::Fi], d)?,
                _ => Vec::new(),
            };
            close(items, at, Kw::Fi)?;
            Ok(Node::If { cond, then, els })
        }
        Kw::While => {
            let cond = opened(items, at, rest, &[Kw::Do], d)?;
            let body = body_of(items, at, Kw::Do, &[Kw::Done], d)?;
            close(items, at, Kw::Done)?;
            Ok(Node::While { cond, body })
        }
        Kw::For => {
            let (name, tail) = first_word(rest);
            let (word_in, words) = first_word(tail);
            if name.is_empty() || word_in != b"in" || !is_name(name) {
                return Err(b"syntax: for NAME in WORDS; do ...; done");
            }
            let body = body_of(items, at, Kw::Do, &[Kw::Done], d)?;
            close(items, at, Kw::Done)?;
            Ok(Node::For { name, words, body })
        }
        _ => Err(b"syntax: a keyword out of place"),
    }
}

/// The condition of an `if` or `while`: the command after the keyword, if
/// any, then the statements up to `until`.
fn opened<'a>(
    items: &[Item<'a>],
    at: &mut usize,
    rest: &'a [u8],
    until: &[Kw],
    d: usize,
) -> Result<Block<'a>, &'static [u8]> {
    let mut cond = Vec::new();
    if !rest.is_empty() {
        cond.push((Conn::Always, Node::Cmd(rest)));
    }
    cond.extend(block(items, at, until, d)?);
    Ok(cond)
}

/// The keyword `k`, the command after it if any, then the block up to `until`.
fn body_of<'a>(
    items: &[Item<'a>],
    at: &mut usize,
    k: Kw,
    until: &[Kw],
    d: usize,
) -> Result<Block<'a>, &'static [u8]> {
    let Some(Item::Kw(Conn::Always, got, rest)) = items.get(*at) else {
        return Err(b"syntax: a block is not closed");
    };
    if *got != k {
        return Err(b"syntax: a keyword out of place");
    }
    *at += 1;
    opened(items, at, rest, until, d)
}

fn close(items: &[Item<'_>], at: &mut usize, k: Kw) -> Result<(), &'static [u8]> {
    match items.get(*at) {
        Some(Item::Kw(Conn::Always, got, rest)) if *got == k && rest.is_empty() => {
            *at += 1;
            Ok(())
        }
        Some(_) => Err(b"syntax: a keyword out of place"),
        None => Err(b"syntax: a block is not closed"),
    }
}

fn next_kw(items: &[Item<'_>], at: usize) -> Option<Kw> {
    match items.get(at) {
        Some(Item::Kw(_, k, _)) => Some(*k),
        _ => None,
    }
}

/// A variable name: a letter or `_`, then letters, digits or `_`.
pub fn is_name(n: &[u8]) -> bool {
    matches!(n.first(), Some(c) if c.is_ascii_alphabetic() || *c == b'_')
        && n.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_')
}
