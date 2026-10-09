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
use alloc::vec::Vec;

use crate::catalog::folders::{in_folder, ROOTS};
use crate::catalog::media::MediaItem;

/// The longest search the field keeps; it is one line in the top bar.
pub const QUERY_MAX: usize = 48;

pub struct Browse {
    pub items: Vec<MediaItem>,
    pub view: Vec<usize>,
    pub sel: usize,
    pub scroll: usize,
    pub scanned: bool,
    /// Why no folder could be listed, when none could; the screens say so in
    /// place of their empty-library words.
    pub scan_error: Option<&'static str>,
    pub query: String,
    /// The root the Folders page shows, by index into `ROOTS`; `None` is all.
    pub folder: Option<usize>,
    pub grid: bool,
}

impl Default for Browse {
    fn default() -> Browse {
        Browse::new()
    }
}

impl Browse {
    pub fn new() -> Browse {
        Browse {
            items: Vec::new(),
            view: Vec::new(),
            sel: 0,
            scroll: 0,
            scanned: false,
            scan_error: None,
            query: String::new(),
            folder: None,
            grid: true,
        }
    }

    pub fn len(&self) -> usize {
        self.view.len()
    }

    pub fn is_empty(&self) -> bool {
        self.view.is_empty()
    }

    pub fn get(&self, slot: usize) -> Option<&MediaItem> {
        self.items.get(*self.view.get(slot)?)
    }

    /// The library entry for `path`, whatever the search or folder shows.
    pub fn item_by_path(&self, path: &str) -> Option<&MediaItem> {
        self.items.iter().find(|m| m.path == path)
    }

    pub fn item_by_path_mut(&mut self, path: &str) -> Option<&mut MediaItem> {
        self.items.iter_mut().find(|m| m.path == path)
    }

    pub fn reindex(&mut self) {
        self.view.clear();
        for (i, item) in self.items.iter().enumerate() {
            let shown = self.folder.is_none_or(|f| in_folder(&item.path, f));
            if shown && matches(item, &self.query) {
                self.view.push(i);
            }
        }
        if self.sel >= self.view.len() {
            self.sel = 0;
            self.scroll = 0;
        }
    }

    /// Types one printable byte into the search, refiltering from the top.
    pub fn type_byte(&mut self, byte: u8) -> bool {
        if !(0x20..=0x7E).contains(&byte) || self.query.len() >= QUERY_MAX {
            return false;
        }
        self.query.push(byte as char);
        self.refilter();
        true
    }

    /// Removes the last character of the search.
    pub fn erase(&mut self) -> bool {
        if self.query.pop().is_none() {
            return false;
        }
        self.refilter();
        true
    }

    /// Shows one root folder, or every folder for `None`.
    pub fn set_folder(&mut self, folder: Option<usize>) -> bool {
        let folder = folder.filter(|&f| f < ROOTS.len());
        if self.folder == folder {
            return false;
        }
        self.folder = folder;
        self.refilter();
        true
    }

    fn refilter(&mut self) {
        self.sel = 0;
        self.scroll = 0;
        self.reindex();
    }
}

fn matches(item: &MediaItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let name = item.name.as_bytes();
    let needle = query.as_bytes();
    if needle.len() > name.len() {
        return false;
    }
    name.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle))
}
