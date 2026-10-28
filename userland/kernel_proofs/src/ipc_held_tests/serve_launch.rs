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

//! K.5: MkAppLaunch admits a capsule SERVES lists for its own listing alone,
//! so the window starts its core and nothing else; AppInstall and the shell
//! launch any listing, as before.

use super::app_launch_rule::{admit_launch, Launch::*};
use super::common::owner;
use super::serve_rule::serves::{LISTINGS, SERVES};

const CORE: &str = "linux.kali.nonos-simplex-chat";

#[test]
fn a_served_capsule_launches_its_own_listing_alone() {
    let window = owner(&["app.simplex", "endpoint.app.simplex.reply"]);
    assert_eq!(admit_launch(false, &window, CORE, LISTINGS), Served);
    for other in ["linux.jq", "linux.kali.nonos-simplex-chat2", "linux.kali.simplex", ""] {
        assert_eq!(admit_launch(false, &window, other, LISTINGS), Refused, "{other}");
    }
    for mine in [&[][..], &["app.simplex.1"], &["app.terminal"], &["app.linux.run.net"]] {
        assert_eq!(admit_launch(false, owner(mine), CORE, LISTINGS), Refused, "{mine:?}");
    }
    for listing in [CORE, "linux.jq"] {
        assert_eq!(admit_launch(true, owner(&[]), listing, LISTINGS), Free, "{listing}");
        assert_eq!(admit_launch(true, &window, listing, LISTINGS), Free, "{listing}");
    }
}

#[test]
fn each_served_capsule_launches_one_listing() {
    let served: Vec<&str> = SERVES.iter().map(|&(capsule, _)| capsule).collect();
    let launched: Vec<&str> = LISTINGS.iter().map(|&(capsule, _)| capsule).collect();
    assert_eq!(launched, served, "a capsule launches without a pairing, or twice");
    assert!(LISTINGS.iter().all(|&(_, listing)| listing.starts_with("linux.")));
}
