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

//! A whole isolation container from a family and its bound device proof,
//! with any section replaceable so a test can write what a forger would.

use nonos_proof_abi::{encode, tag, Kind};

use super::device::DAY;

/// The sections, owned, in tag order.
pub struct Parts {
    pub kind: Kind,
    pub window: u64,
    pub sections: Vec<(u16, Vec<u8>)>,
}

impl Parts {
    pub fn set(&mut self, t: u16, bytes: Vec<u8>) {
        if let Some(s) = self.sections.iter_mut().find(|s| s.0 == t) {
            s.1 = bytes;
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let refs: Vec<(u16, &[u8])> = self.sections.iter().map(|(t, b)| (*t, &b[..])).collect();
        encode(self.kind, self.window, &refs).expect("a well-formed container")
    }
}

pub fn container(
    record: &[u8],
    device: &[u8],
    isolation: &[u8],
    publics: &[u64; 8],
    meas: &[[u8; 32]],
    policy: &[u8],
) -> Parts {
    let words: Vec<u8> = publics.iter().flat_map(|w| w.to_le_bytes()).collect();
    Parts {
        kind: Kind::Isolation,
        window: DAY,
        sections: vec![
            (tag::RECORD, record.to_vec()),
            (tag::DEVICE, device.to_vec()),
            (tag::ISOLATION, isolation.to_vec()),
            (tag::PUBLICS, words),
            (tag::MEASUREMENTS, meas.concat()),
            (tag::POLICY, policy.to_vec()),
        ],
    }
}
