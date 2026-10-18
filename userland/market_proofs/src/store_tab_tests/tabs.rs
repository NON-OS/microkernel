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

//! Which tab holds a tier or a Linux package, and what an empty tab says.

use crate::store::listing::{Listing, Source};
use crate::store::tab::{Tab, TABS};

use super::tiers::{index, shown};

#[test]
fn qwen_tiers_are_models_not_linux_packages() {
    let all = index();
    assert!(all.iter().all(|l| l.source == Source::Model));
    assert_eq!(shown(Tab::Models, &all), 17);
    assert_eq!(shown(Tab::Linux, &all), 0, "no tier under Linux");
    assert_eq!(shown(Tab::All, &all), 17);
    assert_eq!(shown(Tab::NonOs, &all), 0);
    assert_eq!(shown(Tab::Community, &all), 0);
    assert!(TABS.contains(&Tab::Models));
}

#[test]
fn a_linux_package_stays_under_linux() {
    let jq = Listing::new(b"linux.jq".to_vec(), [0; 32], b"jq".to_vec(), true);
    assert!(jq.source == Source::Linux);
    assert!(Tab::Linux.accepts(jq.source) && !Tab::Models.accepts(jq.source));
}

#[test]
fn an_empty_tab_says_why_and_where_to_look() {
    let linux = String::from_utf8(Tab::Linux.empty().to_vec()).unwrap();
    assert!(linux.contains("no Linux packages") && linux.contains("Models"), "{linux}");
    for tab in TABS {
        let said = String::from_utf8(tab.empty().to_vec()).unwrap();
        for line in said.lines() {
            assert!(line.len() <= 60, "{line:?}");
        }
    }
}
