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

use alloc::string::String;

use super::ffi::{free, njs_eval_to_string};
use super::lifecycle::Engine;

impl Engine {
    /// Evaluate a script and return its result coerced to a string, or the
    /// exception message. Pending jobs are drained before the result is read.
    pub fn eval(&self, code: &str) -> String {
        unsafe {
            let p = njs_eval_to_string(self.ctx, code.as_ptr(), code.len());
            if p.is_null() {
                return String::new();
            }
            let mut n = 0;
            while *p.add(n) != 0 {
                n += 1;
            }
            let out = String::from_utf8_lossy(core::slice::from_raw_parts(p, n)).into_owned();
            free(p);
            out
        }
    }
}
