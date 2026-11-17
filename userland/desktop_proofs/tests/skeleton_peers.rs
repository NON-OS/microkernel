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

//! An app waits at start only for the desktop peers it talks to. The toolkit
//! service is drawn through by no app since its render and theme operations
//! were retired, so a late or missing toolkit capsule must not keep an app
//! from opening.

const REQUIRE: &str = include_str!("../../app_skeleton/src/discover/require.rs");
const PEERS: &str = include_str!("../../app_skeleton/src/discover/peers.rs");

#[test]
fn an_app_waits_for_the_compositor_wm_and_input_router_only() {
    for name in ["b\"compositor\"", "b\"wm\"", "b\"input_router\""] {
        assert!(REQUIRE.contains(name), "{name} is no longer awaited");
    }
    assert!(!REQUIRE.contains("b\"toolkit\""), "the toolkit service is awaited again");
    assert!(!PEERS.contains("toolkit"), "Peers carries the toolkit port again");
}
