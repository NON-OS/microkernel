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

//! Pages of packed files already read and checked, kept for every descriptor
//! that opens the same file: a font or library opened again, by this program
//! or the next one, is not read and checked again. A file is named by its
//! digest, so a page kept under it is that file's page and no other's. Only
//! installed software is packed, never a person's own files. The least
//! recently used files go first once BUDGET is passed.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cell::RefCell;

const BUDGET: usize = 96 << 20;

pub type Pages = BTreeMap<usize, Vec<u8>>;

struct Shelf(RefCell<Vec<([u8; 16], Pages, usize)>>);
// SAFETY: the serve loop is the only thread that reads a guest's files.
unsafe impl Sync for Shelf {}

static SHELF: Shelf = Shelf(RefCell::new(Vec::new()));

/// `f` over the pages kept for the file `digest` names, which then becomes
/// the most recently used.
pub fn with<R>(digest: &[u8; 16], f: impl FnOnce(&mut Pages) -> R) -> R {
    let mut shelf = SHELF.0.borrow_mut();
    let mut entry = match shelf.iter().position(|(d, _, _)| d == digest) {
        Some(i) => shelf.remove(i),
        None => (*digest, Pages::new(), 0),
    };
    let out = f(&mut entry.1);
    /* Counted in whole pages: cheap at every read, and the budget is rough. */
    entry.2 = entry.1.len() * nonos_linux_pack::PAGE;
    shelf.push(entry);
    let mut total: usize = shelf.iter().map(|e| e.2).sum();
    while total > BUDGET && shelf.len() > 1 {
        total -= shelf.remove(0).2;
    }
    out
}
