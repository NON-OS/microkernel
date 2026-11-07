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

//! A rig whose device has a cursor queue: a ring region of its own and a
//! part on the driver's yield that serves it as QEMU does, reading the one
//! descriptor of each command and writing only the used entry. It logs
//! every command it took, and can be cued to take commands and answer none.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{fence, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_gfx_wire::{ops, Cursor, Surface};
use nonos_libc::on_yield;

use super::ask::{ask, surface};
use super::rig::{rig, Rig};
use crate::constants::{VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_REGION_SIZE, VQ_USED_OFFSET};
use crate::device::cmd::cursor::REQ_LEN;
use crate::device::virtqueue::{CursorQueue, QueueLayout};
use crate::display::cursor::{Plane, IMAGE};
use crate::regs::Regs;
use crate::tests::harness::DOORBELL;
use crate::tests::model::{view, QUEUE_SIZE};
use crate::tests::part::ring::{bytes, desc, put, region_at, u16_at};

/// One cursor command as the device read it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Said {
    pub kind: u32,
    pub scanout: u32,
    pub x: i32,
    pub y: i32,
    pub resource: u32,
    pub hot: (u32, u32),
    /// The bytes the descriptor gave.
    pub len: u32,
}

#[derive(Default)]
pub struct CursorHost {
    pub said: Vec<Said>,
    /// From now on commands are taken and never used.
    pub dead: bool,
    seen: u16,
    used: u16,
}

impl CursorHost {
    pub fn serve(&mut self, bar: &FakeBar) {
        while self.seen != u16_at(bar, VQ_AVAIL_OFFSET + 2) {
            fence(Ordering::Acquire);
            let slot = VQ_AVAIL_OFFSET + 4 + (self.seen % QUEUE_SIZE) as usize * 2;
            let head = u16_at(bar, slot);
            self.seen = self.seen.wrapping_add(1);
            let d = desc(bar, head, VQ_DESC_OFFSET);
            let at = region_at(bar, d.addr, d.len as usize).expect("a command in the region");
            let b = bytes(bar, at, d.len as usize);
            let word = |i: usize| u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
            self.said.push(Said {
                kind: word(0),
                scanout: word(6),
                x: word(7) as i32,
                y: word(8) as i32,
                resource: word(10),
                hot: (word(11), word(12)),
                len: d.len,
            });
            if self.dead {
                continue;
            }
            let entry = VQ_USED_OFFSET + 4 + (self.used % QUEUE_SIZE) as usize * 8;
            put(bar, entry, &(head as u32).to_le_bytes());
            put(bar, entry + 4, &0u32.to_le_bytes());
            self.used = self.used.wrapping_add(1);
            fence(Ordering::Release);
            put(bar, VQ_USED_OFFSET + 2, &self.used.to_le_bytes());
        }
    }
}

pub struct CursorRig {
    pub r: Rig,
    pub cursor: Rc<RefCell<CursorHost>>,
    _ring: Arc<FakeBar>,
}

/// The one-panel rig with a cursor queue the driver was handed at bring-up.
pub fn cursor_rig() -> CursorRig {
    cursor_rig_on(rig())
}

/// `r` with a cursor queue, as bring-up hands one on.
pub fn cursor_rig_on(mut r: Rig) -> CursorRig {
    let ring = Arc::new(FakeBar::new(VQ_REGION_SIZE));
    let layout = QueueLayout::new(QUEUE_SIZE, view(&ring, 0, VQ_REGION_SIZE), ring.base())
        .expect("a valid layout");
    let regs = Regs { doorbell: DOORBELL + 4, ..r.d.server.display.q.regs };
    r.d.server.display.cursor = Some(Plane::new(CursorQueue::new(layout, regs)));
    let cursor = Rc::new(RefCell::new(CursorHost::default()));
    let (host, region) = (Rc::clone(&r.host), Arc::clone(&r.region));
    let (c, cring) = (Rc::clone(&cursor), Arc::clone(&ring));
    on_yield(move || {
        assert!(!host.borrow_mut().serve(&region), "the driver is killed");
        c.borrow_mut().serve(&cring);
    });
    CursorRig { r, cursor, _ring: ring }
}

/// SURFACE for output 0, its cursor slot filled with `px`.
pub fn shaped(r: &mut Rig, px: u32) -> Surface {
    let s = surface(r, 0);
    assert_ne!(s.cursor_offset, 0, "a cursor slot");
    let base = r.d.server.slots[0].current.expect("a primary").region.base;
    let m = r.d.server.platform.memory.get_mut(&base).expect("its memory");
    let at = s.cursor_offset as usize;
    m[at..at + IMAGE].chunks_exact_mut(4).for_each(|c| c.copy_from_slice(&px.to_le_bytes()));
    s
}

/// CURSOR on output 0: the status the compositor reads.
pub fn point(r: &mut Rig, c: Cursor) -> i32 {
    ask(r, ops::CURSOR, 0, &c.encode()).0
}

/// A cursor at (x, y), its 32 x 32 shape's hot spot at `hot`.
pub fn at(flags: u32, (x, y): (i32, i32), hot: (u16, u16)) -> Cursor {
    Cursor { flags, x, y, hot_x: hot.0, hot_y: hot.1, width: 32, height: 32 }
}

/// What every cursor command is: the whole struct, no more.
pub const LEN: u32 = REQ_LEN as u32;
