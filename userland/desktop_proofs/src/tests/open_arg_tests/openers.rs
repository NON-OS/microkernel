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

//! OP_OPEN_WITH starts only an app that opens files: every app Files hands a
//! file to, and nothing stronger, since any process can send it and the
//! shell starts the app as a launcher.

use crate::shell_actions::open_arg::opens_files;

const FILES_TABLE: &str =
    include_str!("../../../../capsule_file_manager/src/fm/open_with_table.rs");
const HANDLER: &str =
    include_str!("../../../../capsule_desktop_shell/src/server/handlers/open_with.rs");

#[test]
fn every_app_files_hands_a_file_to_is_an_opener() {
    let services: Vec<&str> =
        FILES_TABLE.split("service: \"").skip(1).map(|s| &s[..s.find('"').unwrap()]).collect();
    assert!(services.len() >= 4, "{services:?}");
    for s in services {
        assert!(opens_files(s.as_bytes()), "{s}");
    }
}

#[test]
fn nothing_stronger_than_a_viewer_is_started_this_way() {
    for s in [
        "app.install",
        "app.terminal",
        "tool.qwen",
        "app.settings",
        "app.store",
        "app.nonos_wallet",
        "app.process_manager",
        "app.browser",
        "app.about",
        "app.file_manager",
        "app.calculator",
        "app.snake",
        "",
    ] {
        assert!(!opens_files(s.as_bytes()), "{s}");
    }
}

#[test]
fn the_handler_asks_before_it_finds_the_app() {
    let code: String = HANDLER.split_whitespace().collect();
    let gate = "position(|a|a.service==svc&&opens_files(svc));";
    assert!(code.contains(gate), "open_with gates on opens_files");
    assert_eq!(code.matches("LAUNCHER_APPS.iter()").count(), 1, "one lookup, gated");
    let refuse = "letSome(index)=foundelse{refused(sender_pid,svc);";
    assert!(code.contains(refuse), "a refusal is said");
    assert!(code.contains("[CAP-DENY]open-with:pid={sender_pid}app={svc}"));
}
