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
//! The shipped tiers: each runs the chat window on its own first model, and
//! every model file a tier needs is one the personality pins.

use super::pinned_published::TIERS;
use crate::models::apps::{all as apps, app};
use crate::models::pinned::all as pins;

#[test]
fn every_tier_needs_only_pinned_files_and_opens_its_first() {
    assert_eq!(apps().count(), TIERS.len());
    for tier in TIERS {
        assert!(app(&alloc::format!("qwen-{tier}")).is_some(), "{tier}");
    }
    for app in apps() {
        assert_eq!(app.program, b"/bin/qwenchat", "{}", app.name);
        assert!(!app.models.is_empty(), "{}", app.name);
        for m in app.models {
            assert!(pins().any(|p| p.name == *m), "{} needs an unpinned file", app.name);
        }
        let at = app.args.iter().position(|a| *a == b"-m").expect("names its model");
        let model = [&b"/models"[..], app.models[0]].concat();
        assert_eq!(app.args[at + 1], &model[..], "{}", app.name);
        assert!(app.args.windows(2).any(|w| w == [&b"-ui"[..], &b"window"[..]]));
        /*
         * Parts in order, all of them, so the first finds the rest.
         */
        let n = app.models.len();
        for (k, m) in app.models.iter().enumerate().filter(|_| n > 1) {
            let part = alloc::format!("-{:05}-of-{n:05}.gguf", k + 1);
            assert!(m.ends_with(part.as_bytes()), "{}", app.name);
        }
    }
}

#[test]
fn every_pinned_file_belongs_to_a_tier_and_names_are_distinct() {
    for p in pins() {
        let owner = apps().find(|a| a.models.contains(&p.name)).expect("orphan pin");
        assert_eq!(owner.name.strip_prefix("qwen-"), Some(p.tier), "{:?}", p.name);
    }
    for (i, a) in apps().enumerate() {
        assert!(apps().skip(i + 1).all(|b| b.name != a.name));
        assert!(a.name.starts_with("qwen-") && !a.name.contains(':'));
    }
}
