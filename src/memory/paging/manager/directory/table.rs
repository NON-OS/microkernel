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

//! The directory itself: its slots, and how far into them to look.

use core::sync::atomic::{AtomicBool, AtomicUsize};

use super::slot::Slot;

/// Room for more address spaces than the machine runs processes.
pub(crate) const SLOTS: usize = 1024;

pub(crate) static TABLE: [Slot; SLOTS] = [const { Slot::empty() }; SLOTS];
/// One past the highest slot ever filled: no reader looks further.
pub(crate) static HIGH: AtomicUsize = AtomicUsize::new(0);
/// An address space once found no free slot, so a miss proves nothing.
pub(crate) static FULL: AtomicBool = AtomicBool::new(false);
