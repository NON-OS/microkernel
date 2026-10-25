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

//! The PPI read: version and the operations firmware allows, never the
//! submission function; a TPM without PPI, an absent TPM, the 1.2 id.

use super::rig::tpm;
use crate::platform::inventory;
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;
use crate::tpm_ppi::{read_ppi, Ppi};

fn read(aml: &[u8]) -> (Option<Ppi>, u64) {
    let (mut aml, _) = boot(aml, 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let ppi = read_ppi(&mut aml, &mut evaluator(&mut probe), &devices);
    let submitted = num(&call(&mut aml, "\\GSUB", vec![]).unwrap());
    (ppi, submitted)
}

#[test]
fn version_and_allowed_operations() {
    let (ppi, submitted) = read(&tpm("MSFT0101", 0x0F, &[0x07, 0x01]));
    let ppi = ppi.expect("PPI");
    assert_eq!(ppi.functions, 0x107);
    assert_eq!(ppi.version.as_deref(), Some(&b"1.3"[..]));
    assert_eq!(ppi.ops, [(5, 3), (14, 1), (23, 4)]);
    assert_eq!(submitted, 0, "function 2 never called");
}

#[test]
fn without_function_8_or_1() {
    let (ppi, _) = read(&tpm("MSFT0101", 0x0F, &[0x01]));
    let ppi = ppi.expect("PPI with function 0 alone");
    assert_eq!((ppi.version, ppi.ops), (None, vec![]));
}

#[test]
fn no_ppi_or_no_tpm() {
    assert!(read(&tpm("MSFT0101", 0x0F, &[0x00])).0.is_none(), "bit 0 clear: no PPI");
    assert!(read(&tpm("MSFT0101", 0, &[0x07, 0x01])).0.is_none(), "absent");
    assert!(read(&tpm("PNP0C0A", 0x0F, &[0x07, 0x01])).0.is_none(), "not a TPM");
    assert!(read(&tpm("PNP0C31", 0x0F, &[0x07, 0x01])).0.is_some(), "the 1.2 id");
}
