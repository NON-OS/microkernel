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

//! The first word on an install: a market box held to admit, or none, which
//! only a shipped tier may have.

use super::market_tests::{market, policy, ready};
use crate::install::market::check::{gate, without_box};

/* A release with no policy ("-") is a shipped tier's; any package from an
 * index without its release's box fails before anything is fetched. */
#[test]
fn only_a_shipped_tier_installs_without_a_box() {
    let tier = crate::models::apps::all().next().expect("a shipped tier").name;
    assert_eq!(without_box(tier), Ok(()));
    for pkg in ["git", "qwen-no-such-tier", "vim"] {
        assert_eq!(
            without_box(pkg),
            Err("a package from an index needs its release's box"),
            "{pkg}"
        );
        assert!(gate(pkg, None).is_err(), "{pkg}: no box, never installed");
    }
    assert_eq!(gate(tier, None), Ok(None), "a tier: its own table's box");
    let p = policy(1, 2, ready());
    assert_eq!(gate("git", Some(&market(&p, 1, 2))), Ok(Some(p)), "a market box: admitted");
    assert!(
        gate(tier, Some(&market(&p, 3, 2))).is_err(),
        "a box that fails admit fails even a tier"
    );
}
