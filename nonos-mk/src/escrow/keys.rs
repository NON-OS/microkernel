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

//! The seller's key file (plan/29): one line per answer, "unix keyCommit
//! cipherRoot buyer_pk K" in hex, written and synced before the answer
//! leaves, so deliver() can find K and check the terms' cipherRoot against
//! it. Lines older than OFFER_TTL + MAX_DELIVERY are pruned: by then no order
//! can still be delivered. The file is owner-only.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use nonos_nox_config::{MAX_DELIVERY, OFFER_TTL};

pub const KEEP: u64 = OFFER_TTL + MAX_DELIVERY;

fn open(path: &Path, append: bool) -> std::io::Result<std::fs::File> {
    let mut o = OpenOptions::new();
    o.create(true).write(true).append(append).truncate(!append);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut o, 0o600);
    o.open(path)
}

pub struct Kept<'a> {
    pub key_commit: &'a [u8; 32],
    pub cipher_root: &'a [u8; 32],
    pub buyer_pk: &'a [u8; 32],
    pub k: &'a [u8; 32],
}

pub fn record(path: &Path, now: u64, r: &Kept<'_>) -> std::io::Result<()> {
    let mut f = open(path, true)?;
    let hex = |b: &[u8; 32]| hex::encode(b);
    let (c, root, pk, k) = (hex(r.key_commit), hex(r.cipher_root), hex(r.buyer_pk), hex(r.k));
    writeln!(f, "{now} {c} {root} {pk} {k}")?;
    f.sync_all()
}

/// Rewrites the file without lines older than KEEP, or unreadable ones;
/// returns how many went.
pub fn prune(path: &Path, now: u64) -> std::io::Result<usize> {
    let Ok(text) = std::fs::read_to_string(path) else { return Ok(0) };
    let fresh = |l: &&str| {
        l.split(' ')
            .next()
            .and_then(|t| t.parse::<u64>().ok())
            .is_some_and(|t| now.saturating_sub(t) <= KEEP)
    };
    let kept: Vec<&str> = text.lines().filter(fresh).collect();
    let tmp = path.with_extension("prune");
    let mut f = open(&tmp, false)?;
    kept.iter().try_for_each(|l| writeln!(f, "{l}"))?;
    f.sync_all()?;
    std::fs::rename(&tmp, path)?;
    Ok(text.lines().count() - kept.len())
}
