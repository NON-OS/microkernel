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

//! Opening one record the server sent, with the keys that own it.

extern crate alloc;

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;

pub(super) fn open(keys: &TrafficKeys, seq: u64, record: &[u8]) -> Option<Vec<u8>> {
    crate::record_open::open(keys.suite, &keys.server_key, &keys.server_iv, seq, record)
}
