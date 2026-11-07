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
 * What this stack's SYN and SYN-ACK look like on the wire. Without the
 * profile they carry no options and a window of 8192: no mainstream system
 * sends that shape, and a peer that sees no MSS option falls back to 536
 * bytes a segment (RFC 9293 3.7.1). The normalised profile carries the one
 * option this stack implements, an MSS of 1460, and the 29200-byte window
 * Linux sent on its SYNs for most of the 3.x and 4.x series. SACK-permitted,
 * timestamps and window scaling are not offered: the stack implements none of
 * them, and offering one would commit it to behaviour it does not have.
 *
 * The profile is the capsule's default feature `normalised-syn`, signed off
 * as a separate step; a build with
 * --no-default-features sends the old shape.
 */

pub const NORMALISED: bool = cfg!(feature = "normalised-syn");

const KIND_MSS: u8 = 2;
const LEGACY_WINDOW: u16 = 8192;
const NORMALISED_WINDOW: u16 = 29200;

/// The window a SYN or SYN-ACK advertises, and the receive window held
/// until the first segment of data moves it.
pub fn syn_window(normalised: bool) -> u16 {
    if normalised {
        NORMALISED_WINDOW
    } else {
        LEGACY_WINDOW
    }
}

/// The options area of a SYN or SYN-ACK: empty, or the MSS option. Always a
/// whole number of 32-bit words, as the data offset counts them.
pub fn syn_options(normalised: bool, mss: u16) -> ([u8; 4], usize) {
    if !normalised {
        return ([0; 4], 0);
    }
    let m = mss.to_be_bytes();
    ([KIND_MSS, 4, m[0], m[1]], 4)
}
