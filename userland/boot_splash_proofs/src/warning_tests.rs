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
//! The no-IOMMU warning's row: between the attestation panel and the
//! status line, 14 px from the panel and at least 14 from the status line,
//! and on screen, on every mode.

use crate::layout::{warning, STATUS_ROWS};
use crate::layout_fit::layout;

const MODES: [(u32, u32); 7] =
    [(640, 480), (800, 600), (1024, 768), (1280, 720), (1280, 800), (1920, 1080), (3840, 2160)];

#[test]
fn the_row_sits_between_panel_and_status_on_every_mode() {
    for (w, h) in MODES {
        let l = layout(w, h);
        let (p, r, st) = (l.panel, l.warn, l.status);
        assert_eq!(r, (0, p.1 + p.3 + 14, w, STATUS_ROWS), "{w}x{h}");
        assert_eq!(r, warning(p, w), "{w}x{h}");
        assert!(r.1 + r.3 + 14 <= st.1, "{w}x{h}: clear of the status line");
        assert!(r.1 + r.3 <= h, "{w}x{h}: on screen");
    }
}
