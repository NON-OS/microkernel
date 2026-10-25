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

use alloc::vec::Vec;
use spin::Mutex;

use crate::memory::addr::PhysAddr;
use crate::smp::lock_responsive;

use super::super::types::SurfaceHandle;
use super::gate::GATE;
use super::orphans_free::free_orphans;
use super::reclaim_rule::reclaim_in;

pub(super) struct Orphan {
    pub owner: u32,
    pub handle: SurfaceHandle,
    /* The surface's frame list, by page index, for matching mappings. */
    pub frames: Vec<PhysAddr>,
    /* The frames the owner really unmapped, freed by `reclaim`. */
    pub free: Vec<PhysAddr>,
    /* The DMA run the frames belong to, freed through the broker (L35). */
    pub grant: Option<(u64, u64)>,
    /* The exited group's tables, which map the frames until they are freed. */
    pub tables: u64,
}

/*
 * Frames of surfaces the owner unmapped while another process still had
 * them attached, owned by the registry until no attach record is left.
 */
pub(super) static ORPHANS: Mutex<Vec<Orphan>> = Mutex::new(Vec::new());

/* Free every orphaned frame list that no other process maps any more, under
 * GATE; then the DMA runs held for them (L35), once GATE is let go:
 * grant_run's free_unheld takes it itself. Taken inside, every reclaim (a
 * drop_attach, an munmap, a process exit) froze the surface path on every
 * CPU (reclaim_rule.rs). */
pub fn reclaim() {
    reclaim_in(|| lock_responsive(&GATE), |_| free_orphans(), super::grant_run::free_unheld);
}

/* `pid` exits. Its orphans stay listed: the other holders' attach records
 * outlive the owner, so `reclaim` frees each when its last holder lets go. */
pub fn abandon_owner(_pid: u32) {
    reclaim();
}
