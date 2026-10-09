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

use super::state::State;

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        /*
         * A new document is untitled. It used to carry a default file name
         * without reading that file, so the first save replaced whatever the
         * file held with the new text.
         */
        let path = [0u8; 256];
        let mut s = State {
            owner_pid: 0,
            buf: alloc::vec![0; super::state::CAPACITY],
            len: 0,
            caret: 0,
            sel_anchor: None,
            selecting: false,
            scroll_line: 0,
            visible_rows: 1,
            wrap_cols: 80,
            pane_x: 0,
            pane_y: 0,
            pane_w: 0,
            pane_h: 0,
            glyph_advance: super::layout::GLYPH_ADVANCE,
            status: b"Ctrl-O open  Ctrl-S save  Ctrl-E export  Ctrl-C copy  Ctrl-V paste",
            path,
            path_len: 0,
            prompt: None,
            prompt_path: [0u8; 256],
            prompt_len: 0,
            overwrite_armed: false,
            shell_port: 0,
            dirty: false,
            saved: Default::default(),
            undo: alloc::vec::Vec::new(),
            redo: alloc::vec::Vec::new(),
            find_active: false,
            find_buf: alloc::string::String::new(),
            replace_active: false,
            replace_buf: alloc::string::String::new(),
            last_click_ns: 0,
            last_click_x: 0,
            last_click_y: 0,
            font_scale: 2,
            doc: crate::doc::document::Doc::new(),
            pages: alloc::vec::Vec::new(),
            page_metrics: crate::doc::page::PageMetrics {
                width: 760.0,
                height: 980.0,
                margin: 56.0,
            },
            mode: super::mode::mode_for_path(""),
            marks: alloc::vec::Vec::new(),
            aligns: alloc::vec::Vec::new(),
        };
        s.reset_styles();
        s.reflow();
        s
    }
}
