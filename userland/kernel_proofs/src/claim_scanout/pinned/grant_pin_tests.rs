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

//! The grant table's find (grant/one.rs) read whole (L79). An unmap counts
//! itself inside the claim named by its own grant's device and epoch as
//! recorded; with any other key `claim::inside` finds no claim, the unmap
//! runs outside it, and a takeover's release no longer waits for its pages
//! to go: the race the unmap's count closes is open again, with every other
//! check still passing.

use super::super::glue_text::{body, count, source};

const GRANT: &str = "hardware/broker/grant/one.rs";

#[test]
fn an_unmap_is_handed_its_own_grants_device_and_epoch() {
    assert_eq!(count(&source(GRANT), "fn find("), 1, "a second find");
    let want = "{ let grants = GRANTS.lock(); \
        let g = grants.iter().find(|g| g.grant_id == grant_id && g.pid == pid)?; \
        Some((g.device_id, g.claim_epoch)) }";
    assert_eq!(body(GRANT, "find"), want);
}
