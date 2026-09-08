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

//! Row tables for Editing, Auto Save and Language. Every switch here is backed
//! by a persisted bit; the dropdowns have no popup behind them, so they are
//! listed as `Drop` and the painter draws them from the dimmed style.

use super::sect::{Ctl, Section};

// Every row here changes what the editor does. The bit numbers are the ones
// `live.rs` reads, so moving a row means moving a constant with it.
//
// Four rows were removed rather than left drawn and ignored:
//
//   Word wrap. Turning it off needs horizontal scrolling, which the editor
//   does not have, so a reader who switched it off would watch text leave the
//   right edge with no way to reach it. The switch returns when the scrolling
//   does.
//
//   Smart quotes and auto-capitalise. Neither exists, and neither is obviously
//   wanted in an editor that opens source files; adding them to justify a
//   switch would be building backwards from the panel.
//
//   Tab width. It is a dropdown, and dropdowns here have nothing to open.
pub(super) const EDITING: Section = Section {
    head: "Editing",
    rows: &[
        ("Show invisible characters", Ctl::Toggle(1)),
        ("Highlight the current line", Ctl::Toggle(4)),
    ],
};
