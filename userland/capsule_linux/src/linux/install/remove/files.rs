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

//! A package's files going, each with what the machine wrote beside it: the
//! trailer it minted, and for a market program the policy it vouched and that
//! policy's trailer, in the order market/take.rs proves keeps a program that would
//! not go in its box.

use alloc::vec::Vec;

use crate::linux::file::{key, store_found, store_unlink};

/// The files that would not go; gone already is as good as taken out.
pub(super) fn take_out(files: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut unlink = |p: &[u8]| {
        let _ = store_unlink(&key(p));
    };
    let mut found = |p: &[u8]| store_found(&key(p));
    files
        .iter()
        .filter(|at| !super::super::market::take::take_out(at, &mut unlink, &mut found))
        .cloned()
        .collect()
}
