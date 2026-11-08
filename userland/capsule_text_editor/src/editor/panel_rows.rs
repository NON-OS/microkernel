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

//! The rows behind an open panel, each a label and a value, built for the
//! painter, which lays them out as the controls a press is tested against.

use alloc::string::String;
use alloc::vec::Vec;

use super::app::Editor;
use super::panel::{special_labels, Panel};
use super::wordcount::count_rows;

impl Editor {
    pub(super) fn panel_rows(&self, panel: Panel) -> Vec<(&'static str, String)> {
        match panel {
            Panel::WordCount => {
                let doc = &self.docs[self.active.min(self.docs.len().saturating_sub(1))];
                count_rows(&doc.buf[..doc.len])
            }
            Panel::Special => special_labels().into_iter().map(|l| (l, String::new())).collect(),
        }
    }
}
