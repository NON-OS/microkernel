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

//! The honest container every verify test starts from, and the pins a
//! verifier of the fixture's listing would hold.

use nonos_proof_abi::isolation::STATEMENT;
use nonos_proof_abi::trust::{DeviceRoot, Trust};
use nonos_proof_abi::{ctx, Kind};

use super::run::SUP_MEAS;
use super::{container, family, isolation_proof, prove_bound, Family, Parts};

/// The pins, owned, so a test can change one before it borrows them.
#[derive(Clone)]
pub struct Pinned {
    pub boot_root: [u64; 4],
    pub kernel_root: [u64; 4],
    pub registrars: Vec<DeviceRoot>,
    pub supervisors: Vec<[u8; 32]>,
    pub vendor: Vec<[u8; 256]>,
}

impl Pinned {
    pub fn trust(&self) -> Trust<'_> {
        Trust {
            boot_root: self.boot_root,
            kernel_root: self.kernel_root,
            registrars: &self.registrars,
            supervisors: &self.supervisors,
            vendor_policies: &self.vendor,
            isolation: &STATEMENT,
        }
    }
}

pub fn honest() -> (Family, Parts, Pinned) {
    let f = family();
    let rec = f.record.encode();
    let p = prove_bound(ctx(Kind::Isolation, &rec));
    let iso = isolation_proof(&f);
    let parts = container(&rec, &p.section, iso, &f.publics, &f.meas, &f.policy);
    let pinned = Pinned {
        boot_root: p.boot_root,
        kernel_root: p.kernel_root,
        registrars: vec![p.registry],
        supervisors: vec![SUP_MEAS],
        vendor: Vec::new(),
    };
    (f, parts, pinned)
}
