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

//! The store-to-wallet mailbox (op 19, the plan, the plan): one purchase's
//! names, held at most HOLD_MS. A second POST while one waits is EBUSY, so
//! the store says so; TAKE hands the names over once, and they are gone.

use nonos_payment_proto::{Names, EBUSY};

/// How long posted names wait for the wallet: ten minutes.
pub const HOLD_MS: u64 = 600_000;

#[derive(Default)]
pub struct Mailbox {
    held: Option<(Names, u64)>,
}

impl Mailbox {
    pub fn post(&mut self, names: Names, now_ms: u64) -> Result<(), i32> {
        if self.waiting(now_ms) {
            return Err(EBUSY);
        }
        self.held = Some((names, now_ms));
        Ok(())
    }

    /// The waiting names, once; None when none wait or they waited too long.
    pub fn take(&mut self, now_ms: u64) -> Option<Names> {
        let live = self.waiting(now_ms);
        let (names, _) = self.held.take()?;
        live.then_some(names)
    }

    fn waiting(&self, now_ms: u64) -> bool {
        self.held.as_ref().is_some_and(|(_, at)| now_ms.saturating_sub(*at) < HOLD_MS)
    }
}
