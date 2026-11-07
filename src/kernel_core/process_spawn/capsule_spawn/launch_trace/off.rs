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

/*
 * The hooks as a build without `nonos-launch-trace` sees them: a zero-sized
 * handle and empty bodies, inlined away at every call site.
 */
use super::phase::Phase;

pub(crate) struct Trace;

impl Trace {
    #[inline(always)]
    pub(crate) fn begin(_name: &str) -> Self {
        Trace
    }

    #[inline(always)]
    pub(crate) fn mark(&self, _phase: Phase) {}

    #[inline(always)]
    pub(crate) fn queued(&self, _pid: u32) {}
}

#[inline(always)]
pub(crate) fn on_syscall(_pid: u32) {}

#[inline(always)]
pub(crate) fn on_surface(_pid: u32) {}

#[inline(always)]
pub(crate) fn on_frame(_pid: u32) {}

#[inline(always)]
pub(crate) fn on_exit(_pid: u32) {}
