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

use capsule_browser_proofs::browser::ui::page_part::{PagePart, Pick};

use super::fixtures::{bookmarks, downloads, history};
use super::found::{has, id, laid};
use super::kit::{at_scale, geom};

#[test]
fn every_row_opens_where_drawn_and_its_button_removes_at_its_end() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (s, b) = (history(), bookmarks());
        let found = laid(&s, q);
        for k in 0..4 {
            let r = has(&found, PagePart::Open(k, id(&s, k)));
            assert!(r.x == geom(&s).frame.x && r.w == geom(&s).frame.w, "{k} at {q}");
        }
        (0..4).for_each(|i| assert!(has(&found, PagePart::Pill(Pick::Filter(i))).w > 0));
        let found = laid(&b, q);
        for k in 0..3 {
            let (open, rm) = (PagePart::Open(k, id(&b, k)), PagePart::Remove(k, id(&b, k)));
            let (row, rm) = (has(&found, open), has(&found, rm));
            assert!(rm.right() < row.right() && rm.x > row.x + row.w / 2, "{k} at {q}");
            assert!(rm.y >= row.y && rm.bottom() <= row.bottom(), "{k} at {q}");
        }
        let d = downloads();
        let found = laid(&d, q);
        (0..3).for_each(|k| assert!(has(&found, PagePart::Remove(k, id(&d, k))).w > 0));
        assert!(!found.iter().any(|(p, _)| matches!(p, PagePart::Open(..))), "{q}");
    }
}
