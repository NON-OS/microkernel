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

//! What a path or a printed reason is allowed to be, over arbitrary text: a
//! path `safe` accepts cannot leave the release, and `clean` leaves nothing a
//! terminal would act on and changes nothing that is already harmless.

use crate::release::path::safe;
use crate::verdict::text::clean;
use proptest::prelude::*;

// A character a terminal would act on: a control, or a direction mark.
fn acts(c: char) -> bool {
    c.is_control()
        || matches!(c, '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 4096, failure_persistence: None, ..ProptestConfig::default() })]

    #[test]
    fn a_safe_path_stays_inside(p in "[a-z./\\\\\\x00-\\x1f]{0,24}") {
        if safe(&p) {
            prop_assert!(!p.starts_with('/') && !p.contains('\\'));
            prop_assert!(p.split('/').all(|c| !c.is_empty() && c != "." && c != ".."));
            prop_assert!(!p.chars().any(char::is_control));
        }
    }

    #[test]
    fn a_cleaned_reason_carries_no_control_or_direction_mark(s in "\\PC{0,64}|[\\x00-\\x1f\\u{202a}-\\u{202e}\\u{2066}-\\u{2069}a]{0,32}") {
        let c = clean(&s);
        prop_assert_eq!(c.chars().count(), s.chars().count());
        prop_assert!(!c.chars().any(acts));
    }

    #[test]
    fn harmless_text_is_left_as_it_is(s in "[ -~]{0,64}") {
        prop_assert_eq!(clean(&s), s);
    }
}
