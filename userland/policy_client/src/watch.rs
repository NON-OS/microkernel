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

//! Following one enumerated field from a capsule that applies it.
//!
//! A driver that owns a setting has no event to wait on when the store
//! changes, so it asks. `Watch` bounds the asking to once per interval and
//! hands back a value only when it differs from the last one seen, so the
//! caller applies a change once and otherwise leaves its own state alone.

use core::sync::atomic::{AtomicU16, AtomicU64, Ordering};

use nonos_policy_proto::Field;

/// No value seen yet: outside the range a u8 field can hold.
const UNSEEN: u16 = 0x100;

pub struct Watch {
    field: Field,
    every_ms: u64,
    next_ms: AtomicU64,
    last: AtomicU16,
}

impl Watch {
    pub const fn new(field: Field, every_ms: u64) -> Watch {
        Watch { field, every_ms, next_ms: AtomicU64::new(0), last: AtomicU16::new(UNSEEN) }
    }

    /// The field's value when it changed since the last answer, asked at
    /// most once per interval. `None` also while the store is not up.
    pub fn changed(&self, now_ms: u64) -> Option<u8> {
        if now_ms < self.next_ms.load(Ordering::Relaxed) {
            return None;
        }
        self.next_ms.store(now_ms.saturating_add(self.every_ms), Ordering::Relaxed);
        let value = crate::get_u8(crate::lookup()?, self.field)?;
        let was = self.last.swap(value as u16, Ordering::Relaxed);
        (was != value as u16).then_some(value)
    }
}
