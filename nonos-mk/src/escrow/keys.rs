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

//! The seller's key file: one line per answer, keyCommit and K in hex,
//! written and synced before the answer leaves, so deliver() can always find
//! the K a funded order commits to. The file is created owner-only.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use nonos_nox_chunks::key_commit;

use crate::hosts::Host;

pub fn record(path: &Path, k: &[u8; 32]) -> std::io::Result<()> {
    let mut o = OpenOptions::new();
    o.create(true).append(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut o, 0o600);
    let mut f = o.open(path)?;
    writeln!(f, "{} {}", hex::encode(key_commit(&Host, k)), hex::encode(k))?;
    f.sync_all()
}
