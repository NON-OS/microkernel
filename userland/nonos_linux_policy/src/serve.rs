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

//! SERVE_ONE's field, which only a version 2 policy carries: the one port on
//! 127.0.0.1 the program may listen on, and the name of the one capsule the
//! kernel lets reach it. The port is little-endian at byte 217, the name
//! follows it, zero padded, so one field has one encoding.

use crate::error::PolicyError;
use crate::layout::{AT_PAD_SERVE, AT_SERVE};

pub const FIELD_LEN: usize = AT_PAD_SERVE - AT_SERVE;
pub const NAME_MAX: usize = FIELD_LEN - 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServeOne {
    pub port: u16,
    /// 1 to 32 bytes of [a-z0-9._-] from a letter, then zeros.
    pub name: [u8; NAME_MAX],
    pub name_len: u8,
}

impl ServeOne {
    /// The field for `port` and `name`, if a policy may carry them.
    pub fn new(port: u16, name: &[u8]) -> Option<Self> {
        let letter = name.first().is_some_and(u8::is_ascii_lowercase);
        let word = |c: &u8| matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-');
        if port == 0 || name.len() > NAME_MAX || !letter || !name.iter().all(word) {
            return None;
        }
        let mut padded = [0; NAME_MAX];
        padded[..name.len()].copy_from_slice(name);
        Some(Self { port, name: padded, name_len: name.len() as u8 })
    }

    pub fn name(&self) -> &str {
        let name = self.name.get(..usize::from(self.name_len)).unwrap_or(&[]);
        core::str::from_utf8(name).unwrap_or("")
    }

    /// The field from its bytes, refusing a NUL inside the name as well.
    pub fn from_bytes(field: [u8; FIELD_LEN]) -> Result<Self, PolicyError> {
        let (port, name) = field.split_at(2);
        let len = name.iter().position(|&c| c == 0).unwrap_or(NAME_MAX);
        if name[len..].iter().any(|&c| c != 0) {
            return Err(PolicyError::Serve);
        }
        Self::new(u16::from_le_bytes([port[0], port[1]]), &name[..len]).ok_or(PolicyError::Serve)
    }

    pub fn to_bytes(&self) -> [u8; FIELD_LEN] {
        let mut out = [0; FIELD_LEN];
        out[..2].copy_from_slice(&self.port.to_le_bytes());
        out[2..].copy_from_slice(&self.name);
        out
    }
}
