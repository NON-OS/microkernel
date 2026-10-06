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

//! The ports the driver may touch, by the size of the ABAR window the
//! broker mapped. PI names ports from the controller's own word; only those
//! whose whole register block lies in the window are ever read.

use crate::constants::regs::{PORT_BASE, PORT_CI, PORT_STRIDE};
use crate::controller::window::ports_in_window;

fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        300
    } else {
        full
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// One past the last byte of port `i`'s register block.
fn block_end(i: u32) -> u64 {
    u64::from(PORT_BASE) + u64::from(i + 1) * u64::from(PORT_STRIDE)
}

#[test]
fn the_named_windows_reach_the_named_ports() {
    let named = [
        (0u64, 0u32, "nothing mapped"),
        (0x100, 0, "the global registers alone"),
        (0x17f, 0, "port 0 one byte short"),
        (0x180, 0x1, "port 0 exactly"),
        (0x800, 0x0000_3fff, "a 2 KiB window: ports 0 to 13"),
        (0xfff, 0x1fff_ffff, "a 4 KiB window one byte short: ports 0 to 28"),
        (0x1000, 0x3fff_ffff, "a 4 KiB window, QEMU's ABAR: ports 0 to 29"),
        (0x107f, 0x3fff_ffff, "port 30 one byte short"),
        (0x1080, 0x7fff_ffff, "ports 0 to 30"),
        (0x1100, u32::MAX, "the full 32-port register file"),
        (0x2000, u32::MAX, "an 8 KiB window"),
        (u64::MAX, u32::MAX, "anything larger"),
    ];
    for (window, mask, what) in named {
        assert_eq!(ports_in_window(window), mask, "{what}: window {window:#x}");
    }
}

#[test]
fn a_reached_port_has_every_register_the_driver_reads_inside_the_window() {
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for i in 0..rounds(200_000) {
        let window = match i % 3 {
            0 => xorshift(&mut s) % 0x1200,
            1 => block_end((xorshift(&mut s) % 32) as u32) - xorshift(&mut s) % 2,
            _ => xorshift(&mut s),
        };
        let mask = ports_in_window(window);
        for port in 0..32u32 {
            let reached = mask & (1 << port) != 0;
            assert_eq!(reached, block_end(port) <= window, "window {window:#x} port {port}");
            if reached {
                let last = u64::from(PORT_BASE + port * PORT_STRIDE + PORT_CI) + 4;
                assert!(last <= window, "window {window:#x}: port {port} PxCI outside");
            }
        }
    }
}

#[test]
fn a_hostile_pi_never_names_a_port_past_the_window() {
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..rounds(100_000) {
        let pi = xorshift(&mut s) as u32 | 0xc000_0000;
        let window = 0x1000 + (xorshift(&mut s) % 2) * 0x40;
        let walked = pi & ports_in_window(window);
        assert_eq!(walked & 0xc000_0000, 0, "PI {pi:#x}: port 30 or 31 walked in {window:#x}");
        assert_eq!(walked, pi & 0x3fff_ffff, "PI {pi:#x}: a port inside the window dropped");
    }
}
