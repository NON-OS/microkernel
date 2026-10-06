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

//! The `MkBootAttest` record, read. The kernel writes it in
//! `src/security/boot/loader_check/record.rs`; the layout is documented there
//! and a host test encodes with that file and parses with this one.

pub const BOOT_ATTEST_LEN: usize = 80;
const VERSION: u8 = 1;

/// The loader the kernel admitted, and the signed root and epoch it sits under.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AdmittedLoader {
    pub measurement: [u8; 32],
    pub root: [u8; 32],
    pub epoch: u64,
}

/// The kernel's verdict on the bootloader that started it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BootAttest {
    /// The kernel has not run its check yet.
    NotYet,
    /// The TPM's log replays to PCR 4 and the loader it names is enrolled.
    Measured(AdmittedLoader),
    /// No TPM or no log: the loader's own file is enrolled, but nothing measured it.
    SelfReported(AdmittedLoader),
    /// The evidence was there and failed, with the kernel's refusal code.
    Refused(u32),
    /// No boot-root record or no loader trailer reached the kernel.
    NoEvidence,
}

/*
 * Strict: another version, an unknown state, a nonzero reserved byte, a code
 * outside a refusal, a value under a state that admits nothing or an admitted
 * state with nothing in it is refused, so a record from a newer kernel is
 * never half-read as this one.
 */
pub fn parse_boot_attest(r: &[u8]) -> Option<BootAttest> {
    if r.len() != BOOT_ATTEST_LEN || r[0] != VERSION || r[2..4] != [0u8; 2] {
        return None;
    }
    let mut code = [0u8; 4];
    code.copy_from_slice(&r[4..8]);
    let code = u32::from_le_bytes(code);
    let mut epoch = [0u8; 8];
    epoch.copy_from_slice(&r[8..16]);
    let epoch = u64::from_le_bytes(epoch);
    let mut a = AdmittedLoader { measurement: [0; 32], root: [0; 32], epoch };
    a.measurement.copy_from_slice(&r[16..48]);
    a.root.copy_from_slice(&r[48..80]);
    let none = r[8..80].iter().all(|&b| b == 0);
    match (r[1], code, none) {
        (0, 0, true) => Some(BootAttest::NotYet),
        (1, 0, false) => Some(BootAttest::Measured(a)),
        (2, 0, false) => Some(BootAttest::SelfReported(a)),
        (3, c, true) if c != 0 => Some(BootAttest::Refused(c)),
        (4, 0, true) => Some(BootAttest::NoEvidence),
        _ => None,
    }
}
