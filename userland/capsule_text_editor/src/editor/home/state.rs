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

//! Home screen state. The screen is repainted from scratch every frame and the
//! dispatcher hands it no storage of its own, so the selected nav row lives in
//! a process-wide atomic, the same shape `theme` uses for the active palette.

use core::sync::atomic::{AtomicUsize, Ordering};

/// Only the lists with a store behind them. Starred, Shared with me,
/// Templates and Trash were drawn dimmed and did nothing: the editor keeps no
/// stars, shares nothing, has no templates and deletes for good.
pub(super) const NAV_LABELS: [&str; 2] = ["Home", "Recent"];

static NAV: AtomicUsize = AtomicUsize::new(0);

pub(super) struct HomeState {
    pub nav: usize,
}

impl HomeState {
    pub(super) fn load() -> Self {
        Self { nav: NAV.load(Ordering::Relaxed) % NAV_LABELS.len() }
    }

    pub(super) fn select(nav: usize) -> bool {
        nav < NAV_LABELS.len() && NAV.swap(nav, Ordering::Relaxed) != nav
    }
}
