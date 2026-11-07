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

//! termios2, which glibc 2.42 and later ask a terminal with: TCGETS2 and
//! the three that set it are a console's requests as TCGETS's are, and
//! the 44 bytes carry the same settings with their line rates.

use super::requests::{console, KINDS};
use crate::linux::call::ioctl_req::{request, Req};
use crate::linux::console::termios::{Termios, COOKED, SIZE};
use crate::linux::console::termios2::{from2, to2, SIZE2};
use crate::linux::guest::Kind;

const TCGETS2: u64 = 0x802C_542A;
const TCSETS2: u64 = 0x402C_542B;
const TCSETSW2: u64 = 0x402C_542C;
const TCSETSF2: u64 = 0x402C_542D;
/// B38400, B115200 and BOTHER, as c_cflag holds them.
const B38400: u32 = 0o17;
const B115200: u32 = 0o10002;
const BOTHER: u32 = 0o10000;
const CBAUD: u32 = 0o10017;

fn cflag(t: &[u8]) -> u32 {
    u32::from_le_bytes([t[8], t[9], t[10], t[11]])
}

fn with_cflag(mut t: Termios, c: u32) -> Termios {
    t[8..12].copy_from_slice(&c.to_le_bytes());
    t
}

fn speeds(t2: &[u8; SIZE2]) -> (u32, u32) {
    let w = |at: usize| u32::from_le_bytes([t2[at], t2[at + 1], t2[at + 2], t2[at + 3]]);
    (w(SIZE), w(SIZE + 4))
}

fn asked(t: Termios, input: u32, out: u32) -> [u8; SIZE2] {
    let mut t2 = [0u8; SIZE2];
    t2[..SIZE].copy_from_slice(&t);
    t2[SIZE..SIZE + 4].copy_from_slice(&input.to_le_bytes());
    t2[SIZE + 4..].copy_from_slice(&out.to_le_bytes());
    t2
}

#[test]
fn termios2_is_a_consoles_request_on_a_terminal_only() {
    for kind in KINDS {
        for on in [false, true] {
            let yes = on && console(kind);
            for (req, want) in [
                (TCGETS2, Req::GetTermios2),
                (TCSETS2, Req::SetTermios2 { flush: false }),
                (TCSETSW2, Req::SetTermios2 { flush: false }),
                (TCSETSF2, Req::SetTermios2 { flush: true }),
            ] {
                let got = request(kind, on, req);
                assert_eq!(got, if yes { want } else { Req::NotTty }, "{req:#x} on {on}");
            }
        }
    }
    assert_eq!(request(Kind::Stdin, true, (3 << 32) | TCGETS2), Req::GetTermios2);
}

#[test]
fn the_cooked_settings_read_as_38400_each_way() {
    let t2 = to2(&COOKED);
    assert_eq!(SIZE2, 44, "struct termios2 is 44 bytes");
    assert_eq!(&t2[..SIZE], &COOKED[..]);
    assert_eq!(speeds(&t2), (38400, 38400));
    let split = with_cflag(COOKED, (cflag(&COOKED) & !CBAUD) | B115200 | (0o13 << 16));
    assert_eq!(speeds(&to2(&split)), (2400, 115200), "CIBAUD names the input rate");
}

#[test]
fn settings_read_back_are_the_settings_set() {
    let raw = with_cflag(COOKED, (cflag(&COOKED) & !CBAUD) | B115200);
    let t2 = to2(&raw);
    assert_eq!(from2(&t2, &COOKED), Some(raw));
    let mut changed = COOKED;
    changed[12] ^= 0o2; // ICANON off, as a raw-mode program sets it.
    assert_eq!(from2(&to2(&changed), &COOKED), Some(changed));
}

#[test]
fn a_rate_by_number_is_its_named_rate_or_the_one_there_was() {
    let other = (cflag(&COOKED) & !CBAUD) | BOTHER;
    let named = from2(&asked(with_cflag(COOKED, other), 0, 115200), &COOKED).expect("44 bytes");
    assert_eq!(cflag(&named) & CBAUD, B115200);
    let odd = from2(&asked(with_cflag(COOKED, other), 0, 12345), &COOKED).expect("44 bytes");
    assert_eq!(cflag(&odd) & CBAUD, B38400, "no line here: the rate stays as it was");
    assert_eq!(cflag(&odd) & !CBAUD, cflag(&COOKED) & !CBAUD, "the other bits as asked");
}

#[test]
fn only_44_bytes_are_termios2() {
    let t2 = to2(&COOKED);
    assert_eq!(from2(&t2[..SIZE], &COOKED), None);
    assert_eq!(from2(&[t2.as_slice(), &[0]].concat(), &COOKED), None);
    assert_eq!(from2(&[], &COOKED), None);
}

#[test]
fn a_pseudo_terminal_answers_termios2_from_the_same_settings() {
    const PTY: &str = include_str!("../../../../../capsule_linux/src/linux/pty/ioctl.rs");
    assert!(PTY.contains("TCGETS2 => put(guest, arg, &to2(&termios)),"));
    assert!(PTY.contains(
        "TCSETS2 | TCSETSW2 | TCSETSF2 => set_termios2(guest, (n, &termios), r == TCSETSF2, arg),"
    ));
    for (name, v) in [
        ("TCGETS2", 0x802C_542A_u64),
        ("TCSETS2", 0x402C_542B),
        ("TCSETSW2", 0x402C_542C),
        ("TCSETSF2", 0x402C_542D),
    ] {
        let digits = format!("{:X}", v);
        let spelt = format!("const {name}: u64 = 0x{}_{};", &digits[..4], &digits[4..]);
        assert!(PTY.contains(&spelt), "{spelt}");
    }
}

#[test]
fn any_bytes_tcsets2_is_handed_keep_the_rules() {
    use crate::linux::console::termios2_check::{check_bytes, check_set};
    // Every rate code in each place, by number with every named speed and
    // one no rate has, over the cooked settings and settings no one chose.
    let speeds = [0u32, 50, 9600, 38400, 115_200, 4_000_000, 12_345];
    let codes = (0..=0o17).chain(0o10000..=0o10017);
    let mut taken = 0;
    for out in codes.clone() {
        for input in [0, BOTHER, B38400, B115200, out] {
            for &speed in &speeds {
                let t = with_cflag(COOKED, (cflag(&COOKED) & !CBAUD) | out | (input << 16));
                taken += usize::from(check_set(&asked(t, speed, speed), &COOKED).is_some());
            }
        }
    }
    assert_eq!(taken, 32 * 5 * speeds.len(), "44 bytes refused");
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for len in 0..400 {
        let bytes: Vec<u8> = (0..len % 90)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                s as u8
            })
            .collect();
        check_bytes(&bytes);
    }
}
