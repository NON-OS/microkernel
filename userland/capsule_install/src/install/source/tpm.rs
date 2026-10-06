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
 * Whether this machine has a TPM, read by asking it for a key under a label
 * nothing else uses, as the kernel will ask it for the data volume's key on
 * every boot of the installed disk. The key is wiped unread.
 */

use nonos_libc::{machine_key, MACHINE_KEY_NO_TPM, MACHINE_KEY_WRONG_STATE};

const PROBE_LABEL: &[u8] = b"install.tpm-probe";

#[derive(Clone, Copy)]
pub enum Tpm {
    Answers,
    Refuses,
    Absent,
    Silent,
}

impl Tpm {
    pub fn read() -> Tpm {
        match machine_key(PROBE_LABEL) {
            Ok(mut key) => {
                for b in key.iter_mut() {
                    unsafe { core::ptr::write_volatile(b, 0) };
                }
                Tpm::Answers
            }
            Err(MACHINE_KEY_WRONG_STATE) => Tpm::Refuses,
            Err(MACHINE_KEY_NO_TPM) => Tpm::Absent,
            Err(_) => Tpm::Silent,
        }
    }

    /* What the data volume on the installed disk depends on, in its words. */
    pub fn text(self) -> &'static str {
        match self {
            Tpm::Answers => "present; the installed data volume is keyed by it",
            Tpm::Refuses => "present, but it refused a key in this boot state",
            Tpm::Absent => "none found; the installed data volume stays closed",
            Tpm::Silent => "did not answer",
        }
    }
}
