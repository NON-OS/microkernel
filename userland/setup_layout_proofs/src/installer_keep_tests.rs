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

//! The installer's update screens fit as its other screens do: the confirm
//! screen with an update's rows or with how to get the data ready, the
//! first screen with the prepare row and the longest answer under it, and
//! a stopped update's report, each wrapped by the installer's own wrap.

use crate::installer::metrics::confirm_stack;
use crate::installer_layout_tests::{bodies, text, wrapped};
use crate::literals::literals;

/// The longest literal in `file` under `dir`.
fn longest(dir: &str, file: &str) -> String {
    let all = literals(dir, false).into_iter().filter(|l| l.file.ends_with(file));
    all.map(|l| l.text).max_by_key(|t| t.chars().count()).expect(file)
}

#[test]
fn the_update_screens_fit_their_bodies() {
    let not_ready = text("ui/keep_text.rs", "To keep your data");
    let welcome = text("welcome.rs", "Installs the system");
    let answer = longest("../capsule_install/src/install/handover", "answer.rs");
    let stopped = text("failed_text.rs", "Your data volume was not touched, but");
    let retry = longest("../capsule_install/src/install/ui/screens", "failed_text.rs");
    let cancel = "stopped by you before the update was complete";
    assert!(retry.starts_with("Choose this disk again and update"), "{retry}");
    for b in bodies() {
        let (m, w) = (&b.m, b.w);
        let lines = |s: &str| wrapped(m, w, s) * m.line_h;
        let section = |s: &str| 4 * m.unit + m.unit / 2 + lines(s) + m.gap;
        let now = format!("{stopped} 1023.9 GB were written before it stopped.");
        let prepare = 2 * m.gap + m.card_h(6) + m.row_h + m.unit;
        let screens = [
            ("update", confirm_stack(m, 6)[3]),
            ("not ready", confirm_stack(m, 0)[0] + lines(&not_ready) + m.unit + m.line_h),
            ("prepare", lines(&welcome) + prepare + lines(&answer)),
            ("stopped", section(cancel) + section(&now) + section(&retry)),
        ];
        for (name, need) in screens {
            assert!(need <= b.h, "{}: the {name} screen needs {need}, has {}", b.at, b.h);
        }
    }
}
