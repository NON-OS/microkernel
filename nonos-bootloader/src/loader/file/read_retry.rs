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
 * A read through the firmware's disk driver, in smaller transfers when a
 * larger one fails. Some firmware USB stacks refuse a transfer another
 * takes in one request; the store then came from the next disk with one,
 * the installed disk, and a stick boot came up as the installed system.
 * A transfer refused at the smallest size ends the read, so a stick that
 * cannot be read costs three refused transfers, not one per sector.
 */

use uefi::proto::media::block::BlockIO;

/// The transfer sizes, each tried where the one before it was refused.
const STEPS: [usize; 3] = [256 << 10, 64 << 10, 4 << 10];

/// `buf` from block `lba` on, of `block` bytes each. Every size divides
/// the next larger, and the caller holds `block` to divide 4096.
pub(super) fn read_all(blk: &BlockIO, block: u64, lba: u64, buf: &mut [u8]) -> Option<()> {
    read_in(blk, block, lba, buf, 0)
}

fn read_in(blk: &BlockIO, block: u64, lba: u64, buf: &mut [u8], step: usize) -> Option<()> {
    let media = blk.media().media_id();
    let mut done = 0usize;
    while done < buf.len() {
        let take = (buf.len() - done).min(STEPS[step]);
        let at = lba + done as u64 / block;
        let piece = &mut buf[done..done + take];
        if blk.read_blocks(media, at, piece).is_err() {
            if step + 1 == STEPS.len() {
                return None;
            }
            read_in(blk, block, at, piece, step + 1)?;
        }
        done += take;
    }
    Some(())
}
