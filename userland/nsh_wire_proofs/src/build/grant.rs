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

//! A grant, `nsh-grant/1`, as plain fields with a valid base grant as the
//! default and a valid escalation one call away.

#[derive(Clone)]
pub struct GrantB {
    pub magic: [u8; 4],
    pub version: u16,
    pub kind: u8,
    pub reserved: u8,
    pub grant_id: [u8; 16],
    pub parent_id: [u8; 16],
    pub fleet_id: [u8; 16],
    pub subject: [u8; 64],
    pub user: Vec<u8>,
    pub role: Vec<u8>,
    pub caps: Vec<Vec<u8>>,
    pub not_before: u64,
    pub not_after: u64,
    pub issuer: [u8; 32],
    pub approvers: Vec<[u8; 32]>,
}

impl Default for GrantB {
    fn default() -> Self {
        GrantB {
            magic: *b"NSHG",
            version: 1,
            kind: 1,
            reserved: 0,
            grant_id: [0x11; 16],
            parent_id: [0; 16],
            fleet_id: [0xF1; 16],
            subject: [0x5B; 64],
            user: b"ek".to_vec(),
            role: b"auditor".to_vec(),
            caps: vec![b"CoreExec".to_vec(), b"FileSystem".to_vec(), b"IPC".to_vec()],
            not_before: 1_000,
            not_after: 2_000,
            issuer: [0x15; 32],
            approvers: vec![],
        }
    }
}
