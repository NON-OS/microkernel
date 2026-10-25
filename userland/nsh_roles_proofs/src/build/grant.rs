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

//! A valid `nsh-grant/1`. The base grant gives the user `operator` for
//! five hours around `NOW`.

pub use super::approval::ApprovalB;
use super::keys::{key, K};
use super::{Spoil, BASE_ID, FLEET, HOUR, NOW};

#[derive(Debug, Clone)]
pub struct GrantB {
    pub kind: u8,
    pub grant_id: [u8; 16],
    pub parent_id: [u8; 16],
    pub fleet: [u8; 16],
    pub subject: [u8; 64],
    pub user: Vec<u8>,
    pub role: Vec<u8>,
    pub caps: Vec<&'static str>,
    pub not_before_ms: u64,
    pub not_after_ms: u64,
    pub issuer: [u8; 32],
    pub signer: K,
    pub spoil: Spoil,
    /// Laid out in ascending approver order, as the reader requires.
    pub approvals: Vec<ApprovalB>,
}

impl Default for GrantB {
    fn default() -> Self {
        GrantB {
            kind: 1,
            grant_id: BASE_ID,
            parent_id: [0; 16],
            fleet: FLEET,
            subject: key(K::User).subject(),
            user: b"ek".to_vec(),
            role: b"operator".to_vec(),
            caps: vec!["CoreExec", "IPC", "Memory"],
            not_before_ms: NOW - HOUR,
            not_after_ms: NOW + 4 * HOUR,
            issuer: key(K::AuthA).id(),
            signer: K::AuthA,
            spoil: Spoil::None,
            approvals: vec![],
        }
    }
}
