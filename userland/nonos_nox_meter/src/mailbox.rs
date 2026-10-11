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

//! The store-to-wallet mailbox (op 19, the plan, the plan, v1.4.42): one
//! purchase's names from the store, and once ORDER_NEW has answered, the
//! names with their orderId back from the wallet. Each slot holds one, at
//! most HOLD_MS; a second POST while one waits is EBUSY, so the poster says
//! so; a TAKE hands it over once, and it is gone.

use nonos_payment_proto::{Names, Ready, EBUSY};

/// How long a posting waits to be taken: ten minutes.
pub const HOLD_MS: u64 = 600_000;

pub struct Slot<T> {
    held: Option<(T, u64)>,
}

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Slot { held: None }
    }
}

impl<T> Slot<T> {
    pub fn post(&mut self, item: T, now_ms: u64) -> Result<(), i32> {
        if self.waiting(now_ms) {
            return Err(EBUSY);
        }
        self.held = Some((item, now_ms));
        Ok(())
    }

    /// The waiting posting, once; None when none waits or it waited too long.
    pub fn take(&mut self, now_ms: u64) -> Option<T> {
        let live = self.waiting(now_ms);
        let (item, _) = self.held.take()?;
        live.then_some(item)
    }

    fn waiting(&self, now_ms: u64) -> bool {
        self.held.as_ref().is_some_and(|(_, at)| now_ms.saturating_sub(*at) < HOLD_MS)
    }
}

/// 19/0 and 19/1 use `names`; 19/2 and 19/3 use `ready`.
#[derive(Default)]
pub struct Mailbox {
    pub names: Slot<Names>,
    pub ready: Slot<Ready>,
}
