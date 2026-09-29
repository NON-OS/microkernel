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

use crate::pinned::apps::APPS;
use crate::pinned::pinned::PINNED;

#[test]
fn every_tier_needs_only_pinned_files_and_opens_its_first() {
    assert_eq!(APPS.len(), 4);
    for app in APPS {
        assert_eq!(app.program, b"/bin/qwenchat", "{}", app.name);
        assert!(!app.models.is_empty(), "{}", app.name);
        for m in app.models {
            assert!(PINNED.iter().any(|p| p.name == *m), "{} needs an unpinned file", app.name);
        }
        let at = app.args.iter().position(|a| *a == b"-m").expect("names its model");
        let model = [&b"/models"[..], app.models[0]].concat();
        assert_eq!(app.args[at + 1], &model[..], "{}", app.name);
        assert!(app.args.windows(2).any(|w| w == [&b"-ui"[..], &b"window"[..]]));
    }
}

#[test]
fn every_pinned_file_belongs_to_a_tier_and_names_are_distinct() {
    for p in PINNED {
        assert!(APPS.iter().any(|a| a.models.contains(&p.name)), "orphan pin");
    }
    for (i, a) in APPS.iter().enumerate() {
        assert!(APPS[i + 1..].iter().all(|b| b.name != a.name));
        assert!(a.name.starts_with("qwen-") && !a.name.contains(':'));
    }
}
