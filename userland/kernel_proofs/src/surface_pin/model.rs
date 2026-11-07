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

//! Guests map, pin, reprotect, unmap and exit; the compositor attaches and
//! lets go. Each step does what pin_guest.rs, peer_unmap.rs, peer_protect.rs
//! and the exit path do, through the kernel's own rules.

use std::collections::BTreeMap;

use super::pin_rule::{pin_frames, room, GuestPage};
use super::register_rule::may_register;
use super::rights_attach::{attach, Attach};
use super::rights_rule::{grant, revoke_handle, Right};
use super::unmap_rule::{on_unmap, OnUnmap};

/// The compositor's pid; guests are 1, 2 and 3.
pub const COMPOSITOR: u32 = 100;
pub const PAGES: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Owner {
    Free,
    Guest(u32),
    /// The compositor's own memory.
    Compositor,
    /// Kept by the registry for a view after its guest let it go.
    Registry,
}

#[derive(Clone)]
pub struct Surface {
    pub handle: u64,
    pub owner: u32,
    pub base: usize,
    pub frames: Vec<u64>,
    /// The guest's own reference and record: gone once it unpins.
    pub owner_ref: bool,
}

/// A surface's frames the registry keeps for a view after its guest let
/// some or all of them go: `free` is what the guest unmapped, freed once
/// the last view goes.
#[derive(Clone)]
pub struct Orphan {
    pub handle: u64,
    pub owner: u32,
    pub frames: Vec<u64>,
    pub free: Vec<u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct View {
    pub handle: u64,
    pub frame: u64,
    pub write: bool,
}

#[derive(Default)]
pub struct World {
    pub frames: BTreeMap<u64, Owner>,
    /// Each guest's pages: (frame, user, exec).
    pub tables: BTreeMap<u32, Vec<Option<(u64, bool, bool)>>>,
    pub surfaces: Vec<Surface>,
    /// Frames an unmap or exit left for views.
    pub orphans: Vec<Orphan>,
    pub rights: Vec<Right>,
    pub views: Vec<View>,
    next_handle: u64,
    pub refused_after_revoke: u32,
}

impl World {
    pub fn new() -> Self {
        let mut w = World::default();
        for f in 0..64u64 {
            w.frames.insert(f * 4096, if f < 4 { Owner::Compositor } else { Owner::Free });
        }
        for g in 1..=3 {
            w.tables.insert(g, vec![None; PAGES]);
        }
        w.next_handle = 1;
        w
    }

    fn alloc(&mut self, to: Owner) -> Option<u64> {
        let f = self.frames.iter().find(|(_, o)| **o == Owner::Free).map(|(f, _)| *f)?;
        self.frames.insert(f, to);
        Some(f)
    }

    pub fn free(&mut self, f: u64) {
        assert_ne!(self.frames[&f], Owner::Free, "frame {f:#x} freed twice");
        self.frames.insert(f, Owner::Free);
    }

    pub fn map(&mut self, g: u32, page: usize) {
        if self.tables[&g][page].is_some() {
            return;
        }
        if let Some(f) = self.alloc(Owner::Guest(g)) {
            self.tables.get_mut(&g).unwrap()[page] = Some((f, true, false));
        }
    }

    /// MkPeerPin over `n` pages from `base`, named to the compositor.
    pub fn pin(&mut self, g: u32, base: usize, n: usize) -> Option<u64> {
        let pages: Vec<Option<GuestPage>> = (base..base + n)
            .map(|i| {
                let e = self.tables[&g].get(i).copied().flatten()?;
                Some(GuestPage { frame: e.0, user: e.1, exec: e.2 })
            })
            .collect();
        let frames = pin_frames(&pages).ok()?;
        // Every guest here is one supervisor's: they share its room.
        if !room(self.surfaces.len()) {
            return None;
        }
        let mut sorted = frames.clone();
        sorted.sort_unstable();
        let held = self.surfaces.iter().flat_map(|s| s.frames.clone());
        let held = held.chain(self.orphans.iter().flat_map(|o| o.frames.clone()));
        may_register(false, &sorted, held).ok()?;
        let handle = self.next_handle;
        self.next_handle += 1;
        self.surfaces.push(Surface { handle, owner: g, base, frames, owner_ref: true });
        grant(&mut self.rights, Right { handle, pid: COMPOSITOR, write: false });
        Some(handle)
    }

    fn surface(&self, handle: u64) -> Option<usize> {
        self.surfaces.iter().position(|s| s.handle == handle)
    }

    fn viewed(&self, handle: u64) -> bool {
        self.views.iter().any(|v| v.handle == handle)
    }

    /// MkPeerUnpin: no attach again; the guest's reference goes, and the
    /// slot with it when no view holds it.
    pub fn unpin(&mut self, handle: u64) {
        let Some(i) = self.surface(handle) else { return };
        if !self.surfaces[i].owner_ref {
            return;
        }
        revoke_handle(&mut self.rights, handle);
        self.surfaces[i].owner_ref = false;
        if !self.viewed(handle) {
            self.surfaces.remove(i);
        }
    }

    fn unpin_meeting(&mut self, g: u32, from: usize, to: usize) {
        let meeting: Vec<u64> = self
            .surfaces
            .iter()
            .filter(|s| s.owner == g && s.base < to && from < s.base + s.frames.len().max(1))
            .map(|s| s.handle)
            .collect();
        meeting.into_iter().for_each(|h| self.unpin(h));
    }

    /// MkSurfaceAttach by the compositor.
    pub fn attach(&mut self, handle: u64) {
        let owner = self.surface(handle).map(|i| self.surfaces[i].owner);
        let revoked = !self.rights.iter().any(|r| r.handle == handle);
        match attach(&self.rights, owner, handle, COMPOSITOR, |a, b| a == b) {
            Attach::Receiver { write } => {
                let frames = self.surfaces[self.surface(handle).unwrap()].frames.clone();
                frames.into_iter().for_each(|frame| self.views.push(View { handle, frame, write }));
            }
            Attach::Owner => panic!("the compositor owns a guest's surface"),
            Attach::Refused => {
                if owner.is_some() && revoked {
                    self.refused_after_revoke += 1;
                }
            }
        }
    }

    /// The compositor lets a view go: its last view of an orphan frees what
    /// the guest unmapped; of a live slot with no owner reference, the slot.
    pub fn let_go(&mut self, handle: u64) {
        self.views.retain(|v| v.handle != handle);
        if let Some(i) = self.surface(handle) {
            if !self.surfaces[i].owner_ref {
                self.surfaces.remove(i);
            }
        }
        if let Some(at) = self.orphans.iter().position(|o| o.handle == handle) {
            let o = self.orphans.remove(at);
            o.free.into_iter().for_each(|f| self.free(f));
        }
    }

    /// MkPeerProtect: unpinned first, then the pages may run.
    pub fn protect_exec(&mut self, g: u32, page: usize) {
        self.unpin_meeting(g, page, page + 1);
        if let Some(e) = self.tables.get_mut(&g).unwrap()[page].as_mut() {
            e.2 = true;
        }
    }

    /// MkPeerUnmap of `[from, to)`: unpinned first, then claim_owned's
    /// windows and the orphans keep what a view still maps.
    pub fn unmap(&mut self, g: u32, from: usize, to: usize) {
        self.unpin_meeting(g, from, to);
        let mut held: Vec<(u64, bool)> = Vec::new();
        let hits: Vec<u64> = self
            .surfaces
            .iter()
            .filter(|s| s.owner == g && !s.frames.is_empty())
            .filter(|s| s.base < to && from < s.base + s.frames.len())
            .map(|s| s.handle)
            .collect();
        for handle in hits {
            let i = self.surface(handle).unwrap();
            let s = self.surfaces[i].clone();
            let whole = from <= s.base && s.base + s.frames.len() <= to;
            match on_unmap(self.viewed(handle), whole, true) {
                OnUnmap::Free => {
                    self.surfaces.remove(i);
                }
                OnUnmap::Detach => self.surfaces[i].frames.clear(),
                OnUnmap::Orphan => {
                    self.surfaces[i].frames.clear();
                    let (frames, free) = (s.frames.clone(), Vec::new());
                    self.orphans.push(Orphan { handle, owner: g, frames, free });
                    held.extend(s.frames.iter().map(|&f| (f, true)));
                }
                OnUnmap::Keep => held.extend(s.frames.iter().map(|&f| (f, false))),
            }
        }
        for o in self.orphans.iter().filter(|o| o.owner == g) {
            held.extend(o.frames.iter().map(|&f| (f, true)));
        }
        for page in from..to {
            let Some((f, _, _)) = self.tables.get_mut(&g).unwrap()[page].take() else { continue };
            match held.iter().find(|h| h.0 == f) {
                Some(&(_, orphaned)) => {
                    self.frames.insert(f, Owner::Registry);
                    if orphaned {
                        let o = self.orphans.iter_mut().find(|o| o.frames.contains(&f)).unwrap();
                        o.free.push(f);
                    }
                }
                None => self.free(f),
            }
        }
    }

    /// The guest's last thread ends: its surfaces another process maps
    /// become orphans the teardown leaves alone, and so do the frames of its
    /// own orphans not yet listed to free (exit_owned.rs hold_own_orphans);
    /// the rest go, then every frame its tables still map is freed.
    pub fn exit(&mut self, g: u32) {
        let mut kept = Vec::new();
        let mine: Vec<Surface> = self.surfaces.iter().filter(|s| s.owner == g).cloned().collect();
        self.surfaces.retain(|s| s.owner != g);
        for o in self.orphans.iter_mut().filter(|o| o.owner == g) {
            let mapped: Vec<u64> =
                o.frames.iter().copied().filter(|f| !o.free.contains(f)).collect();
            kept.extend(mapped.iter().copied());
            o.free.extend(mapped);
        }
        for s in mine {
            revoke_handle(&mut self.rights, s.handle);
            if self.viewed(s.handle) && !s.frames.is_empty() {
                kept.extend(s.frames.iter().copied());
                let (frames, free) = (s.frames.clone(), s.frames.clone());
                self.orphans.push(Orphan { handle: s.handle, owner: g, frames, free });
            }
        }
        let pages = std::mem::replace(self.tables.get_mut(&g).unwrap(), vec![None; PAGES]);
        for (f, _, _) in pages.into_iter().flatten() {
            if kept.contains(&f) {
                self.frames.insert(f, Owner::Registry);
            } else {
                self.free(f);
            }
        }
    }

    /// What every view reads is a frame its own guest still has, or one the
    /// registry keeps for it: never free, never another owner's.
    pub fn check(&self) {
        for v in &self.views {
            assert!(!v.write, "a view of a guest's frame is writable");
            let owner = self.frames[&v.frame];
            let guest = self.surfaces.iter().find(|s| s.handle == v.handle);
            match owner {
                Owner::Free => {
                    panic!("view of handle {} reads freed frame {:#x}", v.handle, v.frame)
                }
                Owner::Compositor => panic!("a guest's surface holds the compositor's frame"),
                Owner::Guest(g) => {
                    let s = guest.expect("a view of a guest frame names its surface");
                    assert_eq!(g, s.owner, "frame {:#x} reused by another guest", v.frame);
                }
                Owner::Registry => {}
            }
        }
        for (g, pages) in &self.tables {
            for (f, _, _) in pages.iter().flatten() {
                assert_eq!(self.frames[f], Owner::Guest(*g), "guest {g} maps a frame not its own");
            }
        }
        // A frame the registry keeps is a live surface's or listed to free
        // with an orphan; any other is never freed.
        for (f, owner) in &self.frames {
            if *owner == Owner::Registry {
                let live = self.surfaces.iter().any(|s| s.frames.contains(f));
                let listed = self.orphans.iter().any(|o| o.free.contains(f));
                assert!(live || listed, "frame {f:#x} kept for nothing: never freed");
            }
        }
    }

    /// Frames neither free nor the compositor's own: none once every view
    /// has let go and every guest has exited.
    pub fn held(&self) -> usize {
        self.frames.values().filter(|o| !matches!(o, Owner::Free | Owner::Compositor)).count()
    }
}
