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

//! A policy file compiled: the one `--box` word it means, the box
//! capsule_linux makes of that word for a terminal run (in the network role
//! when the file names destinations), and the box's hash.
//! The box and hash come from capsule_linux's own parse, so the hash printed
//! here is the hash on the run's report.

use alloc::vec::Vec;

use crate::boxed::hash::box_hash;
use crate::boxed::preset::BoxPolicy;
use crate::boxed::terminal_run::for_terminal;
use crate::format::{Base, Policy};

/// The longest box word this reader writes.
pub const MAX_WORD: usize = 255;

pub struct Compiled {
    pub word: Vec<u8>,
    pub policy: BoxPolicy,
    pub hash: [u8; 32],
}

/// The canonical word: `strict`, `offline`, or `allow:` and the folders
/// sorted, once each, then each destination as `net:<it>`, with commas
/// between. capsule_linux sorts the destinations itself.
pub fn word(policy: &Policy) -> Vec<u8> {
    match policy.base {
        Base::Strict => b"strict".to_vec(),
        Base::Offline => b"offline".to_vec(),
        Base::Folders => {
            let mut folders = policy.writes.clone();
            folders.sort();
            folders.dedup();
            let nets = policy.connects.iter().map(|c| [&b"net:"[..], c].concat());
            let items: Vec<Vec<u8>> = folders.into_iter().chain(nets).collect();
            let mut out = b"allow:".to_vec();
            out.extend_from_slice(&items.join(&b','));
            out
        }
    }
}

pub fn compile(policy: &Policy) -> Result<Compiled, &'static str> {
    let word = word(policy);
    if word.len() > MAX_WORD {
        return Err("the box word would be over 255 bytes; name fewer or shorter folders");
    }
    let policy = for_terminal(&word, !policy.connects.is_empty())?;
    let hash = box_hash(&policy);
    Ok(Compiled { word, policy, hash })
}
