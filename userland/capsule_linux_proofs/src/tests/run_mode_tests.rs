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

//! A run request's words and a tier's arguments in each mode: only the
//! exact third word "cli" runs on the terminal, and there the chat keeps
//! its model and loses its window.

use crate::pinned::apps::all;
use crate::run_mode::{parse, Mode};

#[test]
fn a_run_request_names_its_package_and_mode() {
    let got = |a: &[u8]| parse(a).map(|(n, m)| (n, m));
    assert_eq!(got(b"run\0qwen-small"), Some(("qwen-small".into(), Mode::Window)));
    assert_eq!(got(b"run\0qwen-small\0cli"), Some(("qwen-small".into(), Mode::Cli)));
    assert_eq!(got(b"run\0qwen-large\0cli\0"), Some(("qwen-large".into(), Mode::Cli)));
    for other in [&b"run\0qwen-small\0CLI"[..], b"run\0qwen-small\0cli2", b"run\0qwen-small\0"] {
        assert_eq!(got(other).map(|(_, m)| m), Some(Mode::Window), "{other:?}");
    }
    for refused in [&b""[..], b"run", b"run\0", b"install\0qwen-small\0cli", b"run\0\xff"] {
        assert_eq!(got(refused), None, "{refused:?}");
    }
}

#[test]
fn a_tier_on_the_terminal_keeps_its_model_and_drops_its_window() {
    for app in all() {
        let window = Mode::Window.tier_args(app.args);
        assert_eq!(window, app.args.iter().map(|a| a.to_vec()).collect::<Vec<_>>());
        let cli = Mode::Cli.tier_args(app.args);
        assert!(!cli.iter().any(|a| a == b"-ui" || a == b"window"), "{}", app.name);
        let model = [&b"/models"[..], app.models[0]].concat();
        assert_eq!(cli, [b"-m".to_vec(), model], "{}", app.name);
    }
}

#[test]
fn only_the_ui_flag_and_its_word_are_dropped() {
    let args: [&[u8]; 5] = [b"-t", b"4", b"-ui", b"window", b"-v"];
    let cli = Mode::Cli.tier_args(&args);
    assert_eq!(cli, [b"-t".to_vec(), b"4".to_vec(), b"-v".to_vec()]);
    // A trailing flag with no word after it is still dropped, alone.
    assert_eq!(Mode::Cli.tier_args(&[b"-m", b"x", b"-ui"]), [b"-m".to_vec(), b"x".to_vec()]);
}
