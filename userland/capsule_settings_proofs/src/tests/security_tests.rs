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

//! The Security page's machine key row: it says what the kernel answered
//! when the page asked, and nothing on the page is a stored flag nobody sets.
//! Its lock screen block: the passphrase row asks the shell, and its three
//! settings are each read by the router or the shell.

use crate::settings::machine_key::{classify, said, MachineKey, NO_TPM, WRONG_STATE};
use crate::settings::schema::blocks_for;
use crate::settings::schema::rows::{Action, Live, Row, Tone};
use nonos_policy_proto::Field;
use crate::settings::section::Section;

#[test]
fn each_kernel_answer_is_its_own_state() {
    assert_eq!(classify(Ok(())), MachineKey::Ready);
    assert_eq!(classify(Err(NO_TPM)), MachineKey::NoTpm);
    assert_eq!(classify(Err(WRONG_STATE)), MachineKey::BootChanged);
    assert_eq!(classify(Err(-5)), MachineKey::Refused);
    assert_eq!(classify(Err(-110)), MachineKey::TimedOut);
    assert_eq!(classify(Err(-22)), MachineKey::Failed);
}

#[test]
fn only_a_key_given_reads_as_good() {
    assert!(said(MachineKey::Ready).1 == Tone::Ok);
    for k in [MachineKey::NoTpm, MachineKey::BootChanged, MachineKey::TimedOut, MachineKey::Refused, MachineKey::Failed] {
        let (words, tone) = said(k);
        assert!(tone == Tone::Warn, "{words}");
        assert!(!words.is_empty());
    }
    // Before the page has asked, the row claims nothing.
    assert!(said(MachineKey::Unasked) == ("--", Tone::Idle));
}

#[test]
fn security_shows_the_probe_and_no_stored_flag() {
    let blocks = blocks_for(Section::Security);
    let rows: Vec<Row> = blocks.iter().flat_map(|b| b.rows.iter().copied()).collect();
    assert!(rows.iter().any(|r| matches!(r, Row::Live(_, Live::MachineKey))));
    // `SystemKeysGenerated` was a switch nothing ever set: the only fields
    // here are the lock screen's, which the router and the shell read.
    let lock = [Field::AutoLockTimeout, Field::LockOnLid, Field::LockInhibitHolds];
    for r in &rows {
        if let Row::Field(f) = r {
            assert!(lock.contains(f), "a stored field on Security that is not the lock's");
        }
    }
    for f in lock {
        assert!(rows.iter().any(|r| matches!(r, Row::Field(g) if *g == f)));
    }
}

#[test]
fn the_passphrase_is_asked_of_the_shell_not_typed_here() {
    let rows: Vec<Row> =
        blocks_for(Section::Security).iter().flat_map(|b| b.rows.iter().copied()).collect();
    assert!(rows.iter().any(|r| matches!(r, Row::Action(_, Action::LockPassphrase))));
    assert!(rows.iter().find(|r| matches!(r, Row::Action(..))).is_some_and(|r| r.is_stop()));
    let actions = include_str!("../../../capsule_settings/src/settings/actions.rs");
    assert!(actions.contains("call(OP_LOCK_PROMPT, &[], &mut [])"), "no passphrase in the request");
    let shell = include_str!("../../../capsule_settings/src/settings/access/shell.rs");
    assert!(shell.contains("pub const OP_LOCK_PROMPT: u16 = 0x0011;"));
    assert!(shell.contains("pub const OP_LOCK_STANDING: u16 = 0x0012;"));
    let ops = include_str!("../../../capsule_desktop_shell/src/protocol/ops.rs");
    assert!(ops.contains("pub const OP_LOCK_PROMPT: u16 = 0x0011;"));
    assert!(ops.contains("pub const OP_LOCK_STANDING: u16 = 0x0012;"));
}
