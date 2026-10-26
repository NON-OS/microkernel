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

//! One record, one receipt: a RECEIPT job makes the one receipt it ever
//! makes, answers the same ask with it again (a reply lost on the way), and
//! refuses any other ask, so no second doc is ever asked over its record; and
//! the launcher it binds is the asker's own entry in the kernel's registry.

use nonos_proof_abi::wire::RECEIPT;

use crate::job_receipt_entry::{measure_of, ENTRY_LEN};
use crate::job_receipt_step::{step, Made, Step, EBUSY, EPERM};

#[test]
fn a_job_makes_one_receipt_and_answers_only_its_ask_again() {
    let made = Some(Made { seal_no: 9, asked: b"ask" });
    assert_eq!(step(true, RECEIPT, b"ask", true, made), Step::Again, "a lost reply, asked again");
    assert_eq!(step(true, RECEIPT, b"ask", false, made), Step::Again, "nothing new is made");
    assert_eq!(step(true, RECEIPT, b"other", true, made), Step::Refuse(EBUSY), "never a second");
    assert_eq!(step(false, RECEIPT, b"ask", true, made), Step::Refuse(EPERM));
}

fn entry(pid: u32, m: u8) -> Vec<u8> {
    [&pid.to_be_bytes()[..], &[m; 32], &[0xff; 8], &[0]].concat()
}

#[test]
fn the_launcher_is_the_askers_one_entry() {
    let regs = [entry(3, 1), entry(42, 7), entry(5, 2)].concat();
    assert_eq!(entry(1, 1).len(), ENTRY_LEN);
    assert_eq!(measure_of(&regs, 42), Some([7; 32]));
    assert_eq!(measure_of(&regs, 4), None, "no entry");
    assert_eq!(measure_of(&[regs.clone(), entry(42, 8)].concat(), 42), None, "two entries");
    assert_eq!(measure_of(&regs[..ENTRY_LEN + 10], 42), None, "a cut entry");
}
