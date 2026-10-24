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

//! No policy store on the host: every read answers that it has none, so
//! the apps show their defaults.

use nonos_policy_proto::{Field, Header, IPC_PAYLOAD_MAX};

pub fn lookup() -> Option<u32> {
    None
}

pub fn get_u8(_port: u32, _field: Field) -> Option<u8> {
    None
}

pub fn get_bool(_port: u32, _field: Field) -> Option<bool> {
    None
}

pub fn get_i8(_port: u32, _field: Field) -> Option<i8> {
    None
}

pub fn get_str(_port: u32, _field: Field, _out: &mut [u8]) -> Option<usize> {
    None
}

pub struct Reply<'a> {
    pub header: Header,
    pub payload: &'a [u8],
}

pub fn call_within<'a>(
    _port: u32,
    _op: u16,
    _field: u32,
    _kind: u8,
    _rx: &'a mut [u8; IPC_PAYLOAD_MAX],
    _timeout_ms: u64,
) -> Option<Reply<'a>> {
    None
}
