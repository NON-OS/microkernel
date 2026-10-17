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
//! One command carried out on the host, answered OK or refused. The model
//! refuses what the text it was written from does not allow: an id that is
//! taken or missing, a rect outside its resource, a backing attached twice
//! or missing, and (strictly) unreferencing a resource still shown or still
//! backed.

use super::gpu::{Gpu, Res, POISON};
use super::words::{fits, w};
use crate::constants::{
    VG_CMD_GET_DISPLAY_INFO, VG_CMD_RESOURCE_ATTACH_BACKING, VG_CMD_RESOURCE_CREATE_2D,
    VG_CMD_RESOURCE_DETACH_BACKING, VG_CMD_RESOURCE_FLUSH, VG_CMD_RESOURCE_UNREF,
    VG_CMD_SET_SCANOUT, VG_CMD_TRANSFER_TO_HOST_2D, VG_MAX_SCANOUTS,
};

impl Gpu {
    /// Carried out, or refused (false), with the target named for the log.
    pub fn apply(&mut self, kind: u32, b: &[u8]) -> (bool, u32, u32) {
        match kind {
            VG_CMD_GET_DISPLAY_INFO => (true, 0, 0),
            VG_CMD_RESOURCE_CREATE_2D => {
                let (id, rw, rh) = (w(b, 0), w(b, 8), w(b, 12));
                let free = id != 0 && rw > 0 && rh > 0 && !self.res.contains_key(&id);
                let px = vec![POISON; if free { (rw * rh) as usize } else { 0 }];
                if free {
                    self.res.insert(id, Res { w: rw, h: rh, px, backing: None });
                }
                (free, id, 0)
            }
            VG_CMD_RESOURCE_UNREF => {
                let id = w(b, 0);
                let shown = self.scanouts.contains(&id);
                let ok = self.res.get(&id).is_some_and(|r| r.backing.is_none()) && !shown;
                (ok && self.res.remove(&id).is_some(), id, 0)
            }
            VG_CMD_SET_SCANOUT => {
                let (s, id) = (w(b, 16), w(b, 20));
                let ok = (s as usize) < VG_MAX_SCANOUTS
                    && (id == 0 || self.res.get(&id).is_some_and(|r| fits(r, b)));
                if ok {
                    self.scanouts[s as usize] = id;
                }
                (ok, id, s)
            }
            VG_CMD_RESOURCE_FLUSH => {
                let id = w(b, 16);
                (self.res.get(&id).is_some_and(|r| fits(r, b)), id, 0)
            }
            VG_CMD_TRANSFER_TO_HOST_2D => (self.transfer(b), w(b, 24), 0),
            VG_CMD_RESOURCE_ATTACH_BACKING => (self.attach(b), w(b, 0), 0),
            VG_CMD_RESOURCE_DETACH_BACKING => {
                let id = w(b, 0);
                let r = self.res.get_mut(&id).filter(|r| r.backing.is_some());
                (r.map(|r| r.backing = None).is_some(), id, 0)
            }
            _ => (false, 0, 0),
        }
    }
}
