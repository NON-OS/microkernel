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

use super::fixtures::tabs;
use super::found::{has, id, laid};
use super::kit::at_scale;

#[test]
fn every_tab_card_its_close_and_its_chip_act_where_drawn() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let t = tabs(false);
        let found = laid(&t, q);
        for i in 0..5 {
            let k = id(&t, i);
            let (card, x) = (PagePart::Tab(i, k), PagePart::Close(i, k));
            let chip = PagePart::Chip(i, k);
            let (card, x, chip) = (has(&found, card), has(&found, x), has(&found, chip));
            assert!(x.x > card.x + card.w / 2 && x.y < card.y + card.h / 2, "{i} at {q}");
            assert!(chip.x < card.x + card.w / 2 && chip.y > card.y + card.h / 2, "{i} at {q}");
        }
        let found = laid(&tabs(true), q);
        let (name, delete) = (has(&found, PagePart::Name), has(&found, PagePart::Delete));
        assert!(name.right() < delete.x && name.bottom() <= delete.y, "{q}");
        assert!(has(&found, PagePart::Pill(Pick::NewGroup)).w > 0, "{q}");
    }
}
