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
//! Seeded byte streams from the keyboard and the aux port: noise, prefixes,
//! command replies and gaps. The keyboard's framing must agree with a model
//! of scan set 1's (E0 marks one key, E1 opens a six-byte Pause, FA and FE
//! answer a command wherever they land and are never key bytes), and a
//! command reply anywhere must leave the keys unchanged. The aux framing
//! must keep every event in bounds and decode the first whole packet after
//! a pause exactly, whatever noise came before it.

use crate::keymap::flags::{FLAG_BREAK, FLAG_E0_PREFIX, FLAG_E1_PREFIX};
use crate::keymap::translate::translate;
use crate::mouse::parser::MouseParser;
use crate::mouse::ring::MouseRing;
use crate::prefix::{Prefix, Scan};

const SEEDS: u64 = 2_000;
const BYTES: usize = 256;

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A keyboard stream: mostly noise, prefixes and replies a quarter of it.
fn keyboard_stream(seed: u64) -> Vec<u8> {
    (0..BYTES as u64)
        .map(|i| {
            let r = mix(seed << 16 | i);
            match r % 16 {
                0 => 0xE0,
                1 => 0xE1,
                2 => 0xFA,
                3 => 0xFE,
                _ => (r >> 8) as u8,
            }
        })
        .collect()
}

fn keys(stream: &[u8]) -> Vec<(u8, u8)> {
    let mut p = Prefix::IDLE;
    stream
        .iter()
        .filter_map(|&b| match p.take(b) {
            Scan::Key { scancode, flags } => Some((scancode, flags)),
            _ => None,
        })
        .collect()
}

/// Scan set 1's framing, written from its description, not from the driver.
fn model(stream: &[u8]) -> Vec<(u8, u8)> {
    let (mut out, mut e0, mut e1_left) = (Vec::new(), false, 0u8);
    for &b in stream.iter().filter(|&&b| b != 0xFA && b != 0xFE) {
        let brk = if b & 0x80 != 0 { FLAG_BREAK } else { 0 };
        if e1_left > 0 {
            e1_left -= 1;
            if e1_left == 0 {
                out.push((b, FLAG_E1_PREFIX | brk));
            }
        } else if b == 0xE0 {
            e0 = true;
        } else if b == 0xE1 {
            (e0, e1_left) = (false, 2);
        } else {
            out.push((b, brk | if std::mem::take(&mut e0) { FLAG_E0_PREFIX } else { 0 }));
        }
    }
    out
}

#[test]
fn the_keyboard_framing_agrees_with_scan_set_one() {
    for seed in 0..SEEDS {
        let s = keyboard_stream(seed);
        let got = keys(&s);
        assert_eq!(got, model(&s), "seed {seed}");
        for &(code, flags) in &got {
            assert!(
                flags & FLAG_E0_PREFIX == 0 || flags & FLAG_E1_PREFIX == 0,
                "E0 and E1 at once"
            );
            let _ = translate(code, flags);
        }
    }
}

#[test]
fn a_command_reply_anywhere_leaves_the_keys_as_they_were() {
    for seed in 0..SEEDS {
        let s = keyboard_stream(seed);
        let quiet: Vec<u8> = s.iter().copied().filter(|&b| b != 0xFA && b != 0xFE).collect();
        assert_eq!(keys(&s), keys(&quiet), "seed {seed}");
    }
}

#[test]
fn a_pause_planted_in_noise_comes_out_as_one_make_and_one_break() {
    const PAUSE: [u8; 6] = [0xE1, 0x1D, 0x45, 0xE1, 0x9D, 0xC5];
    for seed in 0..SEEDS {
        // Noise with no prefix, so the Pause starts on a clean frame, and a
        // reply put inside it.
        let mut s: Vec<u8> =
            keyboard_stream(seed).into_iter().filter(|b| !matches!(b, 0xE0 | 0xE1)).collect();
        let at = (mix(seed) % s.len() as u64) as usize;
        let mut pause = PAUSE.to_vec();
        pause.insert(1 + (seed % 5) as usize, 0xFA);
        s.splice(at..at, pause);
        let pauses: Vec<_> =
            keys(&s).into_iter().filter(|(_, f)| f & FLAG_E1_PREFIX != 0).collect();
        assert_eq!(
            pauses,
            [(0x45, FLAG_E1_PREFIX), (0xC5, FLAG_E1_PREFIX | FLAG_BREAK)],
            "seed {seed}"
        );
    }
}

#[test]
fn aux_noise_keeps_every_event_in_bounds_and_the_next_packet_after_a_pause_exact() {
    for seed in 0..SEEDS {
        let wheel = seed % 2 == 1;
        let (mut p, mut ring) = (MouseParser::new(wheel), MouseRing::new());
        for i in 0..BYTES as u64 {
            let r = mix(seed << 16 | i);
            p.absorb((r >> 8) as u8, i * (r % 3), &mut ring);
        }
        while let Some(ev) = ring.pop() {
            assert!((-256..=255).contains(&ev.dx) && (-255..=256).contains(&ev.dy), "seed {seed}");
            assert_eq!(ev.buttons & !0x07, 0, "seed {seed}");
        }
        // A pause, then one packet: header 0x29 (left, bit 3, Y negative), dx
        // 5, dy byte 0xFD, which is down 3 and reads as dy 3 on the screen.
        let packet: &[u8] = if wheel { &[0x29, 0x05, 0xFD, 0x01] } else { &[0x29, 0x05, 0xFD] };
        for &b in packet {
            p.absorb(b, 1_000_000, &mut ring);
        }
        let ev = ring
            .pop()
            .unwrap_or_else(|| panic!("seed {seed}: the packet after the pause was lost"));
        assert_eq!((ev.dx, ev.dy, ev.buttons), (5, 3, 1), "seed {seed}");
        assert_eq!(ev.dz, if wheel { 1 } else { 0 });
    }
}
