// NONOS Operating System (AGPL-3.0-or-later)
//! A namespace stand-in for the parser: it knows methods by their last
//! NameSeg alone, which is enough for fixtures that never reuse a name, and
//! it records how deep it was told it stands so tests can see enter and leave
//! balance.

use crate::arena::Arena;
use crate::block::Block;
use crate::error::Error;
use crate::grammar::NamePath;
use crate::parse::{load, Scopes};
use std::collections::HashMap;

#[derive(Default)]
pub struct Flat {
    pub methods: HashMap<[u8; 4], u8>,
    pub depth: i32,
    pub deepest: i32,
}

fn last(bytes: &[u8], name: NamePath) -> Option<[u8; 4]> {
    name.count.checked_sub(1).map(|i| name.seg(bytes, i))
}

impl Scopes for Flat {
    fn enter(&mut self, _: &[u8], _: NamePath) {
        self.depth += 1;
        self.deepest = self.deepest.max(self.depth);
    }
    fn leave(&mut self) {
        self.depth -= 1;
    }
    fn declare(&mut self, bytes: &[u8], name: NamePath, args: u8) {
        if let Some(seg) = last(bytes, name) {
            self.methods.insert(seg, args);
        }
    }
    fn arity(&self, bytes: &[u8], name: NamePath) -> u8 {
        last(bytes, name).and_then(|s| self.methods.get(&s).copied()).unwrap_or(0)
    }
}

/// Loads `aml` as a DSDT and returns the arena and its root, or the refusal.
pub fn parse(aml: &[u8]) -> Result<(Arena, u32), Error> {
    parse_with(aml, &mut Flat::default(), Arena::default())
}

pub fn parse_with(aml: &[u8], flat: &mut Flat, mut arena: Arena) -> Result<(Arena, u32), Error> {
    let t = crate::enc::table(b"DSDT", 2, aml);
    let block = Block::new(&t, usize::MAX)?;
    let root = load(&block, &mut arena, flat)?;
    Ok((arena, root))
}
