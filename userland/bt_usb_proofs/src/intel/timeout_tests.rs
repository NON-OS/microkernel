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

//! The model answers at once: but for the looks a long event takes in
//! 16-octet transfers, every pause is the loader waiting for nothing.

use super::rig::{bootloader, run, TYP};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode::Opcode;

#[test]
fn no_bootup_event_is_waited_for_a_second_then_the_part_is_sent_back() {
    let mut m = bootloader(TYP, 1);
    m.boots = false;
    let (r, m, _) = run(m, 0x0032);
    assert_eq!(r, Err(HciError::Timeout(Opcode(0xfc01))));
    assert!((1_000..1_020).contains(&m.pauses), "{} pauses, about a second", m.pauses);
    assert_eq!(m.to_bootloader, 1);
    assert_eq!(m.boot_addr, Some(0x0010_0800), "Intel Reset sent with the boot address");
    assert!(m.ddc.is_empty());
}

#[test]
fn no_secure_send_result_is_waited_for_five_seconds_then_the_part_is_sent_back() {
    let mut m = bootloader(TYP, 1);
    m.expect = usize::MAX;
    let (r, m, _) = run(m, 0x0032);
    assert_eq!(r, Err(HciError::Timeout(Opcode(0xfc09))));
    assert!((5_000..5_020).contains(&m.pauses), "{} pauses, about five seconds", m.pauses);
    assert_eq!((m.to_bootloader, m.boot_addr), (1, None));
}
