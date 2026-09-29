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

#[cfg(not(feature = "hosted"))]
pub(super) use crate::alloc_stubs::free;
#[cfg(feature = "hosted")]
extern "C" {
    pub(super) fn free(p: *mut u8);
}

extern "C" {
    pub(super) fn njs_new_runtime() -> *mut c_void;
    pub(super) fn njs_new_context(rt: *mut c_void) -> *mut c_void;
    pub(super) fn njs_free_context(ctx: *mut c_void);
    pub(super) fn njs_free_runtime(rt: *mut c_void);
    pub(super) fn njs_eval_to_string(ctx: *mut c_void, code: *const u8, len: usize) -> *mut u8;
    pub(super) fn njs_install_dom(ctx: *mut c_void, host: *mut c_void);
    pub(super) fn njs_dispatch_event(ctx: *mut c_void, node: i32, ty: *const u8) -> i32;
    pub(super) fn njs_event_default_prevented() -> i32;
    pub(super) fn njs_take_navigation() -> *const u8;
    pub(super) fn njs_flush_timers(ctx: *mut c_void, now_ms: f64) -> i32;
}
