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

//! Gathering: setup's answers first, since they are what spares the new
//! system its setup, then the Linux guests, then the capsules, each program
//! whole or not at all, while the store has room.

use alloc::string::String;
use alloc::vec::Vec;

use super::carried::Carried;
use super::sets::signed_sets;
use super::setup::carry_answers;
use super::source::CarrySource;
use crate::store::{StoreBuilder, StoreError};

const TREES: [&str; 2] = ["/linux/", "/capsules/"];

pub fn gather(src: &mut dyn CarrySource) -> Carried {
    let mut store = StoreBuilder::new();
    let answers = carry_answers(src, &mut store);
    let (mut programs, mut left_out, mut left_out_bytes, mut skipped) = (0, 0, 0, 0);
    for tree in TREES {
        let listing: Vec<String> = src
            .list(tree)
            .into_iter()
            .filter(|p| p.starts_with(tree) && !p.ends_with('/'))
            .collect();
        for set in signed_sets(&listing) {
            let files: Option<Vec<Vec<u8>>> = set.iter().map(|p| src.read(p)).collect();
            let Some(files) = files else {
                skipped += 1;
                continue;
            };
            let group: Vec<(&str, &[u8])> =
                set.iter().zip(&files).map(|(p, f)| (p.as_str(), f.as_slice())).collect();
            match store.add_all(&group) {
                Ok(()) => programs += 1,
                Err(StoreError::Full) => {
                    left_out += 1;
                    left_out_bytes += files.iter().map(|f| f.len() as u64).sum::<u64>();
                }
                Err(StoreError::BadName | StoreError::Duplicate) => skipped += 1,
            }
        }
    }
    let store = store.finish();
    Carried { store, answers, programs, left_out, left_out_bytes, skipped }
}
