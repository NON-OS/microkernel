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

//! An enrolment certificate, `nsh-enrol/1`, as plain fields.

use super::{put8, put_u16, put_u64, sig};

#[derive(Clone)]
pub struct EnrolB {
    pub magic: [u8; 4],
    pub version: u16,
    pub reserved: u16,
    pub fleet_id: [u8; 16],
    pub host: Vec<u8>,
    pub ak: [u8; 64],
    pub host_keys: [u8; 64],
    pub pcrs: [[u8; 32]; 4],
    pub boot_version: u8,
    pub boot_apps: Vec<[u8; 32]>,
    pub not_after: u64,
    pub authority: [u8; 32],
}

impl Default for EnrolB {
    fn default() -> Self {
        EnrolB {
            magic: *b"NSHE",
            version: 1,
            reserved: 0,
            fleet_id: [0xF1; 16],
            host: b"db-1.eu.example".to_vec(),
            ak: [0xAC; 64],
            host_keys: [0x4B; 64],
            pcrs: [[0], [1], [2], [7]].map(|p| [p[0]; 32]),
            boot_version: 1,
            boot_apps: vec![[0xB0; 32]],
            not_after: 9_000,
            authority: [0xAA; 32],
        }
    }
}

impl EnrolB {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.magic.to_vec();
        put_u16(&mut out, self.version);
        put_u16(&mut out, self.reserved);
        out.extend_from_slice(&self.fleet_id);
        put8(&mut out, &self.host);
        out.extend_from_slice(&self.ak);
        out.extend_from_slice(&self.host_keys);
        self.pcrs.iter().for_each(|p| out.extend_from_slice(p));
        out.extend_from_slice(&[self.boot_version, self.boot_apps.len() as u8]);
        self.boot_apps.iter().for_each(|a| out.extend_from_slice(a));
        put_u64(&mut out, self.not_after);
        out.extend_from_slice(&self.authority);
        sig(&mut out, 0xE7);
        out
    }
}
