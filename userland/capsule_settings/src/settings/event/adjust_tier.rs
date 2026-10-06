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

//! Stepping the "Qwen model" row to the tier `delta` rows on, and setting it
//! in the policy store, which keeps it on a machine that keeps state. The
//! Terminal reads it on its next `qwen`.

use nonos_policy_proto::Field;

use crate::settings::ipc::op_set_str;
use crate::settings::qwen_tier::step;
use crate::settings::state::cache::STRING_CAP;
use crate::settings::state::status::StatusKind;
use crate::settings::state::{cached_value, store_value, FieldValue, State};

use super::report::report;

pub(super) fn adjust_tier(state: &mut State, delta: i32) {
    let field = Field::QwenTier;
    let next = match cached_value(state, field) {
        FieldValue::Str { bytes, len } => step(&bytes[..len], delta),
        _ => step(b"", delta),
    };
    match op_set_str(state.policy_port, field, next) {
        Ok(()) => {
            let mut snapshot = [0u8; STRING_CAP];
            snapshot[..next.len()].copy_from_slice(next);
            store_value(state, field, FieldValue::Str { bytes: snapshot, len: next.len() });
            state.status.set(StatusKind::Ok, b"updated; the next qwen runs it");
        }
        Err(e) => report(state, e),
    }
}
