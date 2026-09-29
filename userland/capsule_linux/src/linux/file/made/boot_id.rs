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

/*
 * The boot id a family sees, and the fresh uuid Linux gives at each read.
 *
 * Neither is the machine's: the boot id is drawn once, when the family
 * first asks, so it stays the same for the family's whole life as Linux's
 * does for a boot, and no two families share one.
 */

use alloc::vec::Vec;
use core::cell::Cell;

struct Once(Cell<Option<[u8; 16]>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Once {}

static BOOT: Once = Once(Cell::new(None));

pub fn boot_id() -> Vec<u8> {
    let id = BOOT.0.get().unwrap_or_else(|| {
        let fresh = random();
        BOOT.0.set(Some(fresh));
        fresh
    });
    format(id)
}

pub fn uuid() -> Vec<u8> {
    format(random())
}

/* Sixteen random bytes as a version 4, variant 1 uuid, as Linux makes one. */
fn random() -> [u8; 16] {
    let mut b = [0u8; 16];
    let _ = nonos_libc::crypto_random(b.as_mut_ptr(), b.len());
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    b
}

fn format(b: [u8; 16]) -> Vec<u8> {
    let hex =
        |r: &[u8]| r.iter().map(|x| alloc::format!("{x:02x}")).collect::<alloc::string::String>();
    alloc::format!(
        "{}-{}-{}-{}-{}",
        hex(&b[..4]),
        hex(&b[4..6]),
        hex(&b[6..8]),
        hex(&b[8..10]),
        hex(&b[10..])
    )
    .into_bytes()
}
