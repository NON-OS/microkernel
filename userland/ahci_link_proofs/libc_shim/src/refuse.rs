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

//! The calls a test has the broker refuse.

use std::cell::{Cell, RefCell};
use std::mem::discriminant;

use crate::calls::Call;

thread_local! {
    /// Each kind refused, with how many of its calls still go through first.
    static REFUSED: RefCell<Vec<(Call, u32)>> = const { RefCell::new(Vec::new()) };
    static SHORT: Cell<u64> = const { Cell::new(0) };
}

/// Have every later DMA grant say it is `bytes` shorter than the memory it
/// maps, as a broker at fault might.
pub fn grant_short(bytes: u64) {
    SHORT.with(|s| s.set(bytes));
}

/// The length a grant of `size` mapped bytes reports.
pub(crate) fn short(size: usize) -> u64 {
    (size as u64).saturating_sub(SHORT.with(Cell::get))
}

/// Refuse every later call of `call`'s kind; a `Command(bits)` refuses the
/// Command writes that carry all of `bits` (every one, for 0).
pub fn refuse(call: Call) {
    refuse_after(call, 0);
}

/// As `refuse`, once `passed` such calls have gone through.
pub fn refuse_after(call: Call, passed: u32) {
    REFUSED.with(|r| r.borrow_mut().push((call, passed)));
}

pub(crate) fn refused(call: Call) -> bool {
    REFUSED.with(|r| {
        let mut hit = false;
        for (_, left) in r.borrow_mut().iter_mut().filter(|(c, _)| matches(*c, call)) {
            match left.checked_sub(1) {
                Some(n) => *left = n,
                None => hit = true,
            }
        }
        hit
    })
}

fn matches(refused: Call, call: Call) -> bool {
    match (refused, call) {
        (Call::Command(bits), Call::Command(word)) => word & bits == bits,
        _ => discriminant(&refused) == discriminant(&call),
    }
}
