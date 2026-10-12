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
//! One USB host's wire, by the port its service registered.

use crate::wire::call;

pub struct Wire {
    pub port: u32,
}

fn done(status: i32) -> Result<(), i32> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

impl Wire {
    pub(crate) fn op(&self, op: u16, body: &[u8], data: &mut [u8]) -> Result<usize, i32> {
        let (status, n) = call(self.port, op, body, data);
        done(status).map(|_| n)
    }
}
