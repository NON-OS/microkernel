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

//! An install's packages, resolved from the index before any is fetched,
//! in the order the install has always taken them: the listed package
//! first, then each need as it is popped, a name met again skipped, a need
//! named only by a provider after the named ones, then install_if's. Known
//! whole before the first byte moves, several can be fetched at once
//! (get/window.rs) and still be placed in this order. Pure, so the host
//! proofs hold it.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::super::index::{Index, Pkg};

/// Why no closure: a name nothing provides, or more than `max` packages.
#[derive(Debug, PartialEq, Eq)]
pub enum Closed {
    NotProvided,
    TooLarge,
}

pub fn closure<'a>(index: &'a Index, name: &str, max: usize) -> Result<Vec<&'a Pkg>, Closed> {
    let (mut wanted, mut later) = (vec![String::from(name)], Vec::new());
    let (mut done, mut names): (Vec<&Pkg>, Vec<String>) = (Vec::new(), Vec::new());
    // Named needs first, then ones named by what provides them (index_find.rs),
    // then what install_if says comes with the closure (install_if.rs).
    while let Some((next, exact)) = index
        .next_need((&mut wanted, &mut later), &names)
        .or_else(|| Some((super::super::install_if::next(index, &names)?, true)))
    {
        // The listed package itself, by its name; its needs by name or provider.
        let pkg = index.resolve(&next, exact).ok_or(Closed::NotProvided)?;
        if names.contains(&pkg.name) {
            continue;
        }
        if done.len() == max {
            return Err(Closed::TooLarge);
        }
        names.push(pkg.name.clone());
        done.push(pkg);
        wanted.extend(pkg.depends.iter().cloned());
    }
    Ok(done)
}
