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

//! The local items (HID 1.11, 6.2.2.8): the usages of the next main item
//! only. A one- or two-octet usage is on the current Usage Page; a
//! four-octet one names its page in the high half.

use super::error::MapError;
use super::item::Item;
use super::span::Span;
use alloc::vec::Vec;

/// Usage runs one main item may carry.
pub const SPANS_MAX: usize = 16;

#[derive(Default)]
pub struct Locals {
    pub spans: Vec<Span>,
    /// A Usage Minimum waiting for its Maximum.
    min: Option<(u16, u16)>,
}

fn usage(it: &Item, page: u16) -> (u16, u16) {
    let page = if it.size == 4 { (it.data >> 16) as u16 } else { page };
    (page, it.data as u16)
}

impl Locals {
    pub fn set(&mut self, it: &Item, page: u16) -> Result<(), MapError> {
        let (page, id) = usage(it, page);
        match it.tag {
            0 => self.push(Span { page, min: id, max: id }),
            1 => self.min.replace((page, id)).map_or(Ok(()), |_| Err(MapError::Shape)),
            2 => match self.min.take() {
                Some((p, min)) if p == page && min <= id => self.push(Span { page, min, max: id }),
                Some(_) => Err(MapError::Value),
                None => Err(MapError::Shape),
            },
            // Designators and strings name things for people; they do not
            // change the report.
            3..=9 => Ok(()),
            _ => Err(MapError::Unknown),
        }
    }

    fn push(&mut self, s: Span) -> Result<(), MapError> {
        if self.spans.len() == SPANS_MAX {
            return Err(MapError::TooMany);
        }
        self.spans.push(s);
        Ok(())
    }

    /// The usages for a main item, the state cleared for the next.
    pub fn take(&mut self) -> Result<Vec<Span>, MapError> {
        if self.min.take().is_some() {
            return Err(MapError::Shape);
        }
        Ok(core::mem::take(&mut self.spans))
    }
}
