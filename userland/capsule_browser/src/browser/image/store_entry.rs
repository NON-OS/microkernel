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
use core::cell::Cell;

use super::store::{Decoded, Store};

pub(super) enum Status {
    Pending,
    Ready(Decoded),
    Failed,
    /* The raster was dropped for budget; the url may be fetched again. */
    Evicted,
}

pub(super) struct Entry {
    pub status: Status,
    /* Largest box (w, h) the page draws this image into, 0 when unknown. */
    pub hint: (u32, u32),
    pub natural: Option<(u32, u32)>,
    /* How the page draws it: IMG_BOX and/or BG_BOX bits, 0 when unknown. */
    pub drawn_as: u8,
    /* The store clock when the raster was last painted. */
    pub used: Cell<u64>,
    #[cfg(not(feature = "harness"))]
    pub revival: super::revival::Revival,
}

impl Store {
    pub(super) fn entry(&mut self, url: &str) -> &mut Entry {
        self.entries.entry(String::from(url)).or_insert_with(|| Entry {
            status: Status::Pending,
            hint: (0, 0),
            natural: None,
            drawn_as: 0,
            used: Cell::new(0),
            #[cfg(not(feature = "harness"))]
            revival: Default::default(),
        })
    }

    pub(crate) fn mark_pending(&mut self, url: &str) {
        let e = self.entry(url);
        if matches!(e.status, Status::Evicted) {
            e.status = Status::Pending;
        }
    }

    pub(crate) fn hint(&self, url: &str) -> (u32, u32) {
        self.entries.get(url).map_or((0, 0), |e| e.hint)
    }

    pub(crate) fn set_failed(&mut self, url: &str) {
        self.entry(url).status = Status::Failed;
    }
}
