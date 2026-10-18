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
//! A surface given back in order: backing detached, resource unreferenced,
//! then the pages unmapped. One the host keeps waits in limbo with its pages
//! mapped (the host may still read them, so they never go back to the
//! kernel), tried again on each mode set and each STATE.

use nonos_libc::mk_dma_unmap;

use super::gpu::Gpu;
use super::surface::Surface;
use crate::device::cmd::{detach_backing, resource_unref};

impl Gpu {
    /// `s` given back, or kept in limbo until the host lets go of it. The
    /// unref is tried even when the detach was refused: a host that holds
    /// the backing refuses it too, and one that never had it lets go.
    pub fn give_back(&mut self, mut s: Surface) {
        let (q, f) = (&self.q, &self.fences);
        if s.attached && detach_backing(q, f.issue(), s.id).is_ok() {
            s.attached = false;
        }
        if resource_unref(q, f.issue(), s.id).is_ok() {
            let _ = mk_dma_unmap(s.map.grant_id);
            return;
        }
        self.limbo.push(s);
    }

    /// Every surface in limbo tried once more.
    pub fn retry_limbo(&mut self) {
        let kept = core::mem::take(&mut self.limbo);
        kept.into_iter().for_each(|s| self.give_back(s));
    }
}
