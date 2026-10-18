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
use super::base::Trb;
/// Read the TRB at `slot_va`, as the controller may be writing it.
///
/// # Safety
/// `slot_va` is a 16-byte aligned TRB slot inside a live ring or event
/// segment grant of this process.
pub unsafe fn read_volatile_at(slot_va: u64) -> Trb {
    // SAFETY: the caller's contract.
    unsafe { core::ptr::read_volatile(slot_va as *const Trb) }
}
