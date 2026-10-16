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

use spin::{Mutex, Once};

use super::book::Book;
use super::frame::Frame;
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::fdt::find::timer_mem::TimerFrame;

/// The frame the tree names, kept at boot while the tree is read.
pub(super) static DESCRIBED: Once<TimerFrame> = Once::new();

/// The frame, once mapped, checked and its interrupt taken.
pub(super) static FRAME: Once<Frame> = Once::new();

/// Every sleeping CPU's deadline. Taken with IRQs masked, from the idle path
/// and from the frame's interrupt, and the frame written under it, so what
/// the frame holds is always the book's earliest.
pub(super) static BOOK: Mutex<Book<MAX_CPUS>> = Mutex::new(Book::new());
