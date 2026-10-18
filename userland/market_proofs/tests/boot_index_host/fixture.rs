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

//! One run of boot_index::load over an operator catalogue of a given length.

use crate::boot_index::load;
use crate::clients::vfs;
use crate::ingest::{self, IngestError};
use crate::store::Store;
use crate::syscall::{self, N_MK_DEBUG};
use crate::verify::AnyVerifier;

pub const PATH: &[u8] = b"/nonos/marketplace/index.bin";
pub const MAX: usize = 8 << 20;
pub const NO_BASELINE: &[u8] =
    b"[MARKET] this image was built with no catalogue; nothing to list\n";

/// Runs load over an operator file of `len` patterned bytes, ingest answering `refusal`.
pub fn load_file(len: usize, refusal: Option<IngestError>) -> (Vec<u8>, Store) {
    let file: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
    vfs::reset();
    syscall::reset();
    ingest::reset(refusal);
    vfs::put(PATH, file.clone());
    let mut store = Store::default();
    load(&mut store, &AnyVerifier);
    (file, store)
}

/// Each line load logged, every one through mk_debug's syscall.
pub fn said() -> Vec<Vec<u8>> {
    let lines = syscall::lines();
    assert!(lines.iter().all(|(num, _)| *num == N_MK_DEBUG));
    lines.into_iter().map(|(_, line)| line).collect()
}
