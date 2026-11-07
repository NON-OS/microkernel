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


//! Isolation preservation (`Formal.isolation_preservation`) on the surface
//! registry's rules: the model's `consent` and `named` fields say a view is
//! the project's or a receiver the project named, that teardown drops a pid's
//! rights (`Surface.drop`), and that the project's release drops every right
//! (`Surface.releaseBy`). Random sequences of the real operations are run
//! against a ghost record of who was ever named.

extern crate alloc;
use alloc::vec::Vec;

use super::rights_attach::{attach, Attach};
use super::rights_rule::{grant, revoke_handle, revoke_pid, share_write, Grant, Right, SHARE_WRITE};
use super::share_rule::{Named, Shares};
use super::words::Stream;

#[test]
fn only_the_owner_or_a_named_group_may_attach() {
    let mut s = Stream::new(0x5a12);
    for _ in 0..2_000 {
        let owner = (s.next() % 6) as u32 + 1;
        let mut shares = Shares::new(owner);
        let mut consented: Vec<u32> = Vec::new();
        for _ in 0..12 {
            let g = (s.next() % 8) as u32;
            if matches!(shares.name(g), Named::Added | Named::Already) && g != owner {
                consented.push(g);
            }
            for x in 0..8u32 {
                if shares.may_attach(x) {
                    assert!(x == owner || consented.contains(&x), "owner {owner} x {x}");
                }
            }
        }
    }
}

#[test]
fn rights_follow_consent_teardown_and_release() {
    let mut s = Stream::new(0x7e57);
    let owner = 1u32;
    for _ in 0..2_000 {
        let mut rights: Vec<Right> = Vec::new();
        let mut consented: Vec<(u64, u32)> = Vec::new();
        for _ in 0..16 {
            let handle = s.next() % 2 + 1;
            let pid = (s.next() % 5) as u32 + 2;
            match s.next() % 4 {
                0 | 1 => {
                    let r = Right { handle, pid, write: s.next() & 1 == 0 };
                    if matches!(grant(&mut rights, r), Grant::New | Grant::Already) {
                        consented.push((handle, pid));
                    }
                }
                2 => {
                    revoke_pid(&mut rights, pid);
                    assert!(rights.iter().all(|r| r.pid != pid));
                }
                _ => {
                    revoke_handle(&mut rights, handle);
                    assert!(rights.iter().all(|r| r.handle != handle));
                }
            }
            for h in 1..=2u64 {
                for p in 1..=6u32 {
                    match attach(&rights, Some(owner), h, p, |a, b| a == b) {
                        Attach::Owner => assert_eq!(p, owner),
                        Attach::Receiver { .. } => assert!(consented.contains(&(h, p))),
                        Attach::Refused => {}
                    }
                }
            }
        }
    }
}

#[test]
fn a_slot_with_no_owner_gives_no_view() {
    let rights = [Right { handle: 1, pid: 2, write: true }];
    assert_eq!(attach(&rights, None, 1, 2, |a, b| a == b), Attach::Refused);
}

/// The write bit of a receiver's view is the one its share asked for
/// (`Surface.attachWrite`); any other flag refuses the share.
#[test]
fn a_share_names_the_write_bit_of_the_view() {
    assert_eq!(share_write(0), Some(false));
    assert_eq!(share_write(SHARE_WRITE), Some(true));
    for flags in 2u64..64 {
        assert_eq!(share_write(flags), None);
    }
}
