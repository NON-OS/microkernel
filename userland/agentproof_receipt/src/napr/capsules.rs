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

//! A receipt's capsules field, walked out of the kernel's TCG2 log by a
//! capsule without nsh_attest: each PCR 14 EV_NONOS_CAPSULE event's 65-byte
//! body, in log order. That they replay to the quoted PCR 14 is checked by
//! the verifier (verify/capsules.rs); a proof holds this walk to nsh_attest's.

use alloc::vec::Vec;

use super::le::Le;
use super::CAPSULE;

const PCR_CAPSULE: u32 = 14;
const EV_NONOS_CAPSULE: u32 = 0x4E4F_4E01;
const MAX_BANKS: usize = 8;

/// The capsule bodies, or None for a log this walk cannot read to its end.
pub fn capsules_of(log: &[u8]) -> Option<Vec<u8>> {
    let mut c = Le(log, 0);
    c.take(4 + 4 + 20)?; // the header's PCR, type and SHA-1-shaped digest
    let banks = banks(c.body()?)?;
    let mut out = Vec::new();
    while c.1 < log.len() {
        let (pcr, kind, n) = (c.u32()?, c.u32()?, c.u32()? as usize);
        (n == banks.len()).then_some(())?;
        for _ in 0..n {
            let alg = c.u16()?;
            c.take(banks.iter().find(|b| b.0 == alg)?.1 as usize)?;
        }
        let body = c.body()?;
        if pcr == PCR_CAPSULE && kind == EV_NONOS_CAPSULE && body.len() == CAPSULE {
            out.extend_from_slice(body);
        }
    }
    Some(out)
}

/// The Spec ID event's banks: (algorithm, digest size).
fn banks(spec: &[u8]) -> Option<Vec<(u16, u16)>> {
    let mut b = Le(spec, 16 + 4 + 4);
    let n = b.u32()? as usize;
    (n <= MAX_BANKS).then_some(())?;
    (0..n).map(|_| Some((b.u16()?, b.u16()?))).collect()
}
