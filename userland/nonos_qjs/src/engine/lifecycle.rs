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

use core::ffi::c_void;

use super::ffi::{
    njs_free_context, njs_free_runtime, njs_install_dom, njs_new_context, njs_new_runtime,
};

pub struct Engine {
    pub(super) rt: *mut c_void,
    pub(super) ctx: *mut c_void,
}

impl Engine {
    /// Create a fresh runtime and context. Returns None if the engine could not
    /// allocate.
    pub fn new() -> Option<Engine> {
        unsafe {
            let rt = njs_new_runtime();
            if rt.is_null() {
                return None;
            }
            let ctx = njs_new_context(rt);
            if ctx.is_null() {
                njs_free_runtime(rt);
                return None;
            }
            Some(Engine { rt, ctx })
        }
    }

    /// Install the `document` global, binding DOM methods to `host`. The host
    /// pointer is handed to every njs_dom_* callback as the node tree to mutate;
    /// it must outlive every eval that touches the DOM.
    pub fn install_dom(&self, host: *mut c_void) {
        unsafe { njs_install_dom(self.ctx, host) }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            njs_free_context(self.ctx);
            njs_free_runtime(self.rt);
        }
    }
}
