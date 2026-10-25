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

//! The kind-2 container over a delivery, with any section replaceable so a
//! test can write what a forger would, and the pins a buyer would hold.

use nonos_proof_abi::{encode, tag, Kind};

use super::delivery::Delivered;
use super::device::DAY;
use super::Pinned;

impl Delivered {
    pub fn bytes_with(&self, rec: &[u8], body: &[u8], meas: &[u8]) -> Vec<u8> {
        let s =
            [(tag::RECORD, rec), (tag::DEVICE, &self.proven.section[..]), (tag::DELIVERY, body)];
        let all: Vec<(u16, &[u8])> = s.into_iter().chain([(tag::MEASUREMENTS, meas)]).collect();
        encode(Kind::Delivery, DAY, &all).expect("a kind-2 container")
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes_with(&self.record.encode(), &self.body.encode(), &self.meas.concat())
    }

    pub fn pinned(&self) -> Pinned {
        let p = self.proven;
        let (boot_root, kernel_root, registrars) = (p.boot_root, p.kernel_root, vec![p.registry]);
        Pinned {
            boot_root,
            kernel_root,
            registrars,
            supervisors: vec![super::run::SUP_MEAS],
            vendor: Vec::new(),
        }
    }
}
