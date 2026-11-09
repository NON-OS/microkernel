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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Controls;

use super::label::label;
use super::strip::{self, Shown};
use crate::browser::state::View;
use crate::browser::tabs::Tabs;
use crate::browser::ui::tabs_geom::{Strip, TabPart};

/// The titlebar tab strip, laid out afresh in `c`. The frame paints behind
/// it. A tab leads with Home on the home page and the globe on a site, and
/// shows it is busy while its page loads.
pub fn paint(tabs: &Tabs, fb: &mut PaintBuffer, c: &mut Controls<TabPart>) {
    let (n, active) = (tabs.list.len(), tabs.active);
    let s = Strip { n, active, w: fb.width as i32, h: fb.height as i32 };
    let labels: Vec<String> = tabs.list.iter().map(|t| label(t)).collect();
    let lead = |home: bool| if home { IconId::Home } else { IconId::Globe };
    let icons: Vec<IconId> = tabs.list.iter().map(|t| lead(t.view == View::Home)).collect();
    let busy: Vec<bool> = tabs.list.iter().map(|t| t.loading()).collect();
    strip::paint(fb, &s, &Shown { labels: &labels, icons: &icons, busy: &busy }, c);
}
