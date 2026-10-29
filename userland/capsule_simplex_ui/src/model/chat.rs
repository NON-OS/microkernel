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

//! One chat as the window holds it.

use alloc::string::String;
use alloc::vec::Vec;

use crate::proto::chat_ref::ChatRef;
use crate::proto::item::Item;
use crate::proto::status::Status;

/// The most items a chat keeps; older ones are dropped as new ones come.
pub const KEEP: usize = 200;

#[derive(Clone, Debug, PartialEq)]
pub struct Chat {
    pub chat: ChatRef,
    pub name: String,
    /// Oldest first.
    pub items: Vec<Item>,
    pub unread: u32,
    /// Its recent items were fetched, not only the last one.
    pub loaded: bool,
}

impl Chat {
    pub fn new(chat: ChatRef, name: String) -> Self {
        Chat { chat, name, items: Vec::new(), unread: 0, loaded: false }
    }

    pub fn last(&self) -> Option<&Item> {
        self.items.last()
    }

    /// Takes an item, replacing the one with its id; true when it is new.
    pub fn put(&mut self, item: Item) -> bool {
        if let Some(have) = self.items.iter_mut().find(|i| i.id == item.id) {
            *have = item;
            return false;
        }
        let at = self.items.iter().rposition(|i| i.id < item.id).map_or(0, |p| p + 1);
        self.items.insert(at, item);
        if self.items.len() > KEEP {
            self.items.remove(0);
        }
        true
    }

    pub fn set_status(&mut self, id: i64, status: Status) {
        if let Some(have) = self.items.iter_mut().find(|i| i.id == id) {
            have.status = status;
        }
    }
}
