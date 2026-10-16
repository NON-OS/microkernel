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

//! Storing a change the compositor took (keep.rs) in the policy store.

use nonos_policy_proto::{IPC_PAYLOAD_MAX, KIND_U8, OP_SET};

use super::arrange::ScreenPage;
use super::keep::kept;
use crate::settings::ipc::call::call;
use crate::settings::schema::access_knob::Knob;

/// Whether the change is kept for the next start; true for a row that
/// keeps nothing.
pub fn store(port: u32, page: &ScreenPage, knob: Knob) -> bool {
    let Some((field, value)) = kept(page, knob) else { return true };
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    call(port, OP_SET, field, KIND_U8, &[value], &mut rx).is_ok()
}
