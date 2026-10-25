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
//! Stands in for the kernel's volatile `Frame`: same bounds, but every store
//! is recorded, so a proof can read back the order a writer used.

use std::sync::Mutex;

use super::out_of_frame::OutOfFrame;

type Stores = Mutex<Vec<(usize, u64)>>;

#[derive(Clone, Copy)]
pub struct Frame {
    stores: &'static Stores,
}

impl Frame {
    pub fn recording() -> Self {
        Self { stores: Box::leak(Box::new(Mutex::new(Vec::new()))) }
    }

    pub fn write(self, index: usize, value: u64) -> Result<(), OutOfFrame> {
        if index >= 512 {
            return Err(OutOfFrame);
        }
        self.stores.lock().expect("store log").push((index, value));
        Ok(())
    }

    pub fn stores(self) -> Vec<(usize, u64)> {
        self.stores.lock().expect("store log").clone()
    }
}
