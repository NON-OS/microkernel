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
use alloc::vec;
use alloc::vec::Vec;

use super::super::super::tree::Dom;
use super::mode::Mode;
use super::state::Builder;

impl Builder {
    /// A builder for a whole document, starting in the "initial" mode.
    pub fn document() -> Self {
        Builder {
            dom: Dom::new(),
            open: Vec::new(),
            open_meta: Vec::new(),
            fmt: Vec::new(),
            mode: Mode::Initial,
            orig: Mode::Initial,
            tmpl: Vec::new(),
            head: None,
            form: None,
            context: None,
            frameset_ok: true,
            foster: false,
            skip_lf: false,
            table_text: String::new(),
            comment_at: (usize::MAX, 0),
            switch_to: None,
            flags: vec![0],
            depth: vec![0],
            open_names: [0; 64],
            attrs_used: 0,
            attr_bytes: 0,
            stopped: false,
        }
    }
}
