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
//! The surface table of a machine with three capsules, driven only through
//! the kernel's rules, in the order the kernel calls them.

use super::handle::{decode_handle, encode_handle};
use super::closing_rule::{admits, close, forget, Closing};
use super::rights_attach::{attach, Attach};
use super::rights_rule::{grant, revoke_handle, revoke_pid, share_write, Grant, Right, SHARE_WRITE};
use super::rule::{Named, Shares};
use crate::process::mmap_va::MmapVa;

pub const SLOTS: usize = 4;

#[derive(Clone, Copy)]
pub struct Slot {
    pub owner: u32,
    pub epoch: u32,
    pub shares: Shares,
}

pub struct Machine {
    pub slots: [Option<Slot>; SLOTS],
    pub epochs: [u32; SLOTS],
    pub rights: Vec<Right>,
    pub closing: Closing<u64>,
    pub va: MmapVa,
}

pub fn handle(idx: usize, epoch: u32) -> u64 {
    encode_handle(idx as u32, epoch)
}

fn decode(h: u64) -> (usize, u32) {
    let (idx, epoch) = decode_handle(h);
    (idx as usize, epoch)
}

impl Machine {
    pub fn new() -> Self {
        Self { slots: [None; SLOTS], epochs: [0; SLOTS], rights: Vec::new(), closing: Vec::new(), va: MmapVa::new() }
    }

    /// `owner` registers a surface: a free slot, its epoch moved on.
    pub fn create(&mut self, owner: u32) -> Option<u64> {
        let idx = self.slots.iter().position(Option::is_none)?;
        self.epochs[idx] += 1;
        let epoch = self.epochs[idx];
        self.slots[idx] = Some(Slot { owner, epoch, shares: Shares::new(owner) });
        Some(handle(idx, epoch))
    }

    fn owned(&mut self, owner: u32, h: u64) -> Option<&mut Slot> {
        let (idx, epoch) = decode(h);
        self.slots.get_mut(idx)?.as_mut().filter(|s| s.epoch == epoch && s.owner == owner)
    }

    /// MkSurfaceShare's flags, decoded as the syscall does, then
    /// share_surface: the right first, then the slot's name.
    pub fn share(&mut self, owner: u32, h: u64, receiver: u32, write: bool) -> bool {
        self.share_flags(owner, h, receiver, if write { SHARE_WRITE } else { 0 })
    }

    pub fn share_flags(&mut self, owner: u32, h: u64, receiver: u32, flags: u64) -> bool {
        let Some(write) = share_write(flags) else { return false };
        if self.owned(owner, h).is_none() {
            return false;
        }
        if grant(&mut self.rights, Right { handle: h, pid: receiver, write }) == Grant::Full {
            return false;
        }
        let Some(slot) = self.owned(owner, h) else { return false };
        matches!(slot.shares.name(receiver), Named::Added | Named::Already)
    }

    /// owner_releases, then the slot goes.
    pub fn release(&mut self, owner: u32, h: u64) {
        if self.owned(owner, h).is_some() {
            revoke_handle(&mut self.rights, h);
            let (idx, _) = decode(h);
            self.slots[idx] = None;
        }
    }

    /// the project's exit: keep_at_exit closes its surfaces to attach, then
    /// the slots are released and the closing entries forgotten.
    pub fn exit_begin(&mut self, pid: u32) {
        let mine: Vec<u64> = (0..SLOTS)
            .filter_map(|i| self.slots[i].filter(|s| s.owner == pid).map(|s| handle(i, s.epoch)))
            .collect();
        close(&mut self.closing, pid, &mine);
        revoke_pid(&mut self.rights, pid);
    }

    pub fn exit_end(&mut self, pid: u32) {
        for i in 0..SLOTS {
            if let Some(s) = self.slots[i].filter(|s| s.owner == pid) {
                revoke_handle(&mut self.rights, handle(i, s.epoch));
                self.slots[i] = None;
            }
        }
        forget(&mut self.closing, pid);
    }

    /// attach_surface: permit, then the rights, then the closing list; on
    /// success, the view's base in the receiver's VA and whether it writes.
    pub fn attach(&mut self, pid: u32, h: u64, pages: u64) -> Option<(u64, bool)> {
        let (idx, epoch) = decode(h);
        let slot = (*self.slots.get(idx)?)?;
        if slot.epoch != epoch || !slot.shares.may_attach(pid) {
            return None;
        }
        let owner = Some(slot.owner);
        let write = match attach(&self.rights, owner, h, pid, |a, b| a == b) {
            Attach::Owner => true,
            Attach::Receiver { write } => write,
            Attach::Refused => return None,
        };
        if !admits(&self.closing, h) {
            return None;
        }
        Some((self.va.reserve_guarded(pages)?, write))
    }
}
