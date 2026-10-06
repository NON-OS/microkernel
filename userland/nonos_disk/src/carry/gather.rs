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
//! system its setup, then the wallpapers they kept, then the Linux guests,
//! then the capsules, each program whole or not at all, while the store has
//! room; then the files the Linux programs read, which a program carried
//! without them could not use.

use alloc::string::String;
use alloc::vec::Vec;

use super::carried::Carried;
use super::sets::{signed_sets, CERT, MANIFEST, TRAILER};
use super::setup::carry_answers;
use super::wallpapers::carry_wallpapers;
use super::source::CarrySource;
use crate::store::{StoreBuilder, StoreError};

const TREES: [&str; 2] = ["/linux/", "/capsules/"];

pub fn gather(src: &mut dyn CarrySource) -> Carried {
    let mut store = StoreBuilder::new();
    let (answers, set) = carry_answers(src, &mut store);
    let wallpapers = carry_wallpapers(src, &mut store, set);
    let (mut programs, mut left_out, mut left_out_bytes, mut skipped) = (0, 0, 0, 0);
    let mut linux = Vec::new();
    for tree in TREES {
        let listing: Vec<String> = src
            .list(tree)
            .into_iter()
            .filter(|p| p.starts_with(tree) && !p.ends_with('/'))
            .collect();
        if tree == "/linux/" {
            linux = listing.clone();
        }
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
    let (mut data, mut data_left_out) = (0, 0);
    for path in linux_data(&linux) {
        let Some(file) = src.read(&path) else { continue };
        match store.add_all(&[(path.as_str(), file.as_slice())]) {
            Ok(()) => data += 1,
            Err(StoreError::Full) => data_left_out += 1,
            Err(_) => {}
        }
    }
    let store = store.finish();
    Carried {
        store,
        answers,
        programs,
        left_out,
        left_out_bytes,
        skipped,
        data,
        data_left_out,
        wallpapers,
    }
}

/// What under /linux/ is neither a program nor a proof: the files programs
/// read. A program, a file with a trailer beside it, travels with its proofs
/// or not at all, and one enrolled on this machine alone not at all, so it
/// and every proof file stay out of this list. Nothing here can run: the
/// personality proves whatever it runs.
fn linux_data(listing: &[String]) -> Vec<String> {
    use alloc::collections::BTreeSet;
    let has: BTreeSet<&str> = listing.iter().map(String::as_str).collect();
    let proof = |p: &str| [TRAILER, CERT, MANIFEST].iter().any(|s| p.ends_with(s));
    let program = |p: &str| {
        let base = p.strip_suffix(".elf").unwrap_or(p);
        has.contains(alloc::format!("{base}{TRAILER}").as_str())
    };
    listing.iter().filter(|p| !proof(p) && !program(p)).cloned().collect()
}
