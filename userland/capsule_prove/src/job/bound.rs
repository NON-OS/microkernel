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

//! The op and pid a job serves, from its argv as init writes it:
//! "--nonos-prove-job", then "prove", "deliver" or "receipt", then the pid in
//! decimal.
//! `None` when argv does not say both, and then the job serves no one.

use alloc::vec::Vec;

use nonos_proof_abi::wire::{DELIVER, PROVE, RECEIPT};

const JOB_ARG: &[u8] = b"--nonos-prove-job";

pub fn parse(a: &[Vec<u8>]) -> Option<(u16, u32)> {
    let at = a.iter().position(|x| x == JOB_ARG)?;
    let op = match a.get(at + 1)?.as_slice() {
        b"prove" => PROVE,
        b"deliver" => DELIVER,
        b"receipt" => RECEIPT,
        _ => return None,
    };
    let pid = core::str::from_utf8(a.get(at + 2)?).ok()?.parse::<u32>().ok()?;
    (pid != 0).then_some((op, pid))
}

/// Whether argv, NUL separated as the kernel hands it over, names a receipt
/// job: the word after the first JOB_ARG, as parse reads it, without a heap.
pub fn receipt(argv: &[u8]) -> bool {
    let mut words = argv.split(|&b| b == 0).skip_while(|w| *w != JOB_ARG).skip(1);
    words.next() == Some(b"receipt".as_slice())
}
