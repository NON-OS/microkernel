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

//! What the catalogue pane promises with no catalogue, and what `o` may open.

use nonos_market_proto::reason::*;

use crate::store::next_step::can_open;
use crate::store::progress::Progress;

/*
 * With no signed catalogue there is no model catalogue either, so nothing
 * installs and nothing downloads: the pane promises only what still works,
 * a model already on the disk, in lines short enough for the list pane.
 */
#[test]
fn no_catalogue_promises_only_a_model_already_here() {
    use crate::store::failure::Failure;
    use nonos_market_proto::status::E_NODATA;
    let said = String::from_utf8(Failure::Status(E_NODATA).catalogue_trouble()).unwrap();
    assert!(said.contains("ENODATA") && said.contains("nothing can be installed"), "{said}");
    assert!(said.contains("already on this disk"), "{said}");
    for line in said.lines() {
        assert!(line.len() <= 60, "{line:?} is {} long", line.len());
    }
}

/* `o` starts what can start: never a package that is not installed. */
#[test]
fn only_what_can_start_is_asked_to_open() {
    assert!(can_open(b"linux.jq", Progress::Installed));
    for p in [Progress::Idle, Progress::Removed, Progress::Failed(WHY_PACKAGE), Progress::Queued] {
        assert!(!can_open(b"linux.jq", p), "{p:?}");
    }
    // A tier's window opens, and says itself when its model is not here.
    assert!(can_open(b"linux.qwen-small", Progress::Idle));
    assert!(can_open(b"linux.qwen-qwen3-4b", Progress::Removed));
}
