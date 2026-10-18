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

//! What the driver should have kept, read off the sink's log by the test's
//! own rule: block by block from the base, as many as declared and at most
//! BLOCKS, each tried at most TRIES times, each try aimed where E-DDC puts
//! the block, the first whole one kept. A try the rule does not call for,
//! or one aimed elsewhere, fails the test.

use super::own::{declared, whole, BLOCK};
use super::sink::Try;

pub const TRIES: usize = 3;
pub const BLOCKS: usize = 4;

pub struct Verdict {
    pub kept: Vec<u8>,
    /// The log ran out while the rule still called for a try.
    pub cut: bool,
    /// Some block was kept on a later try than its first.
    pub retried: bool,
}

/// `whole_end` is the read's deadline, `xfer` one try's.
pub fn judge(log: &[Try], whole_end: u64, xfer: u64) -> Verdict {
    let (mut kept, mut tries, mut retried) = (Vec::new(), log.iter(), false);
    'block: while kept.len() / BLOCK < BLOCKS && kept.len() / BLOCK <= declared(&kept) {
        let n = kept.len() / BLOCK;
        for k in 0..TRIES {
            let Some(t) = tries.next() else { return Verdict { kept, cut: true, retried } };
            aimed(t, n);
            if taken(t, whole_end, xfer) && whole(&t.got, &kept) {
                kept.extend_from_slice(&t.got);
                retried |= k > 0;
                continue 'block;
            }
        }
        break;
    }
    assert!(tries.next().is_none(), "a try the rule does not call for");
    Verdict { kept, cut: false, retried }
}

/// Aimed at block `n`: from block 2 on, segment n / 2 written first, and
/// before it none; then 128 bytes from 0x50 at n % 2 * 128, unless the
/// segment was refused; never a word past them.
fn aimed(t: &Try, n: usize) {
    let seg = (n >= 2).then_some((n / 2) as u8);
    assert_eq!(t.segment.map(|s| s.0), seg, "block {n}'s segment");
    let refused = t.segment.is_some_and(|s| !s.1);
    let want = (!refused).then_some((0x50, (n % 2 * 128) as u32, BLOCK));
    assert_eq!(t.read, want, "block {n}'s read");
    assert!(t.got.len() <= BLOCK, "a word past the bytes asked");
}

/// Taken off the bus: every word, no NAK, and the wait phase shown while
/// the try's deadline still ran (the driver's clock read after it lands
/// 50 us on, as the model's clock moves).
fn taken(t: &Try, whole_end: u64, xfer: u64) -> bool {
    let end = (t.t0 + xfer).min(whole_end);
    t.read.is_some() && !t.nak && t.got.len() == BLOCK && t.phase && t.phase_at + 50 < end
}
