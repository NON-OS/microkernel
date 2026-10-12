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

//! The legacy register file a test sets up: the QueueNumMax it chooses,
//! what the device was told, its status and link word, the doorbells rung.
//! One per test thread.

use std::cell::RefCell;

#[derive(Default)]
pub struct Model {
    pub qmax: [u16; 2],
    pub sel: u16,
    pub pfn: [Option<u32>; 2],
    pub status: u8,
    /// Status reads that still answer non-zero after a reset.
    pub reset_lag: u32,
    pub link: u16,
    pub rung: Vec<u16>,
}

thread_local!(static MODEL: RefCell<Model> = RefCell::new(Model::default()));

/// Run `f` on this thread's register file.
pub fn with<T>(f: impl FnOnce(&mut Model) -> T) -> T {
    MODEL.with(|m| f(&mut m.borrow_mut()))
}

/// A fresh register file whose queues are `qmax` entries long.
pub fn fresh(qmax: u16) {
    with(|m| *m = Model { qmax: [qmax; 2], ..Model::default() });
}
