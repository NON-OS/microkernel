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
//! The router's notice and the shell's reading of it agree: every
//! frame the router sends is read by the shell's own rule as a notice
//! exactly when it is one, and a notice carries no event. The shell is told
//! once a minute, and the refusal holds until the keyboard is re-paired.

use crate::protocol::NOTICE_KIND;
use crate::route::chord::KEY_DELETE;
use crate::route::trusted_hold::told;
use crate::router_tests::router_bond_kit::*;
use crate::router_desk::*;
use crate::shell_router_notice::is_notice;

#[test]
fn the_shell_reads_a_notice_exactly_when_the_router_sent_one() {
    let mut ctx = desk(800, 600);
    chord(&mut ctx, JUST_WORKS);
    chord(&mut ctx, PASSKEY);
    nonos_libc::advance_ms(crate::route::bond::NOTICE_QUIET_MS);
    ctx.trusted = told(SHELL, nonos_libc::mk_uptime_ms());
    key_down(&mut ctx, b'x' as u32, JUST_WORKS);
    key_down(&mut ctx, b'y' as u32, 0);
    let frames = nonos_libc::take_sent();
    let notices: Vec<_> = frames.iter().filter(|(_, f)| is_notice(f)).collect();
    assert_eq!(notices.len(), 2, "one per refusal a minute apart: {frames:?}");
    for (pid, f) in &notices {
        assert_eq!(*pid, SHELL);
        assert_eq!(u16::from_le_bytes([f[8], f[9]]), NOTICE_KIND);
        assert!(f[10..].iter().all(|&b| b == 0), "a notice carried an event");
    }
    assert_eq!(frames.len() - notices.len(), 3, "the passkey chord's two keys and y");
}

#[test]
fn the_shell_is_told_once_a_minute() {
    let mut ctx = desk(800, 600);
    for _ in 0..3 {
        chord(&mut ctx, JUST_WORKS);
    }
    assert_eq!(notices(&out()), [SHELL]);
    nonos_libc::advance_ms(crate::route::bond::NOTICE_QUIET_MS - 1);
    chord(&mut ctx, JUST_WORKS);
    assert_eq!(notices(&out()), []);
    nonos_libc::advance_ms(1);
    chord(&mut ctx, JUST_WORKS);
    assert_eq!(notices(&out()), [SHELL]);
}

#[test]
fn the_refusal_holds_until_the_keyboard_is_paired_authenticated() {
    let mut ctx = desk(800, 600);
    chord(&mut ctx, JUST_WORKS);
    chord(&mut ctx, JUST_WORKS);
    assert_eq!(events(&out()), []);
    chord(&mut ctx, PASSKEY);
    assert_eq!(events(&out()), [(SHELL, KEY_DELETE); 2]);
}
