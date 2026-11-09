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

use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::Pill as Shown;

pub use super::pill_row::row;
use crate::browser::paint::pages::group_pills;
use crate::browser::shell::downloads::counts;
use crate::browser::shell::route::FILTERS;
use crate::browser::shell::{Page, Shell};
use crate::browser::ui::page_part::Pick;

const DOWNLOADS: [&str; 3] = ["All", "Saved", "Failed"];

/// One filter pill: its label, the count it shows if it has one, whether
/// it is the filter in force, the category of its dot, whether it is
/// unavailable, and what a click on it does.
pub struct Pill {
    pub label: String,
    pub count: Option<usize>,
    pub on: bool,
    pub dot: Option<Category>,
    pub dim: bool,
    pub pick: Pick,
}

impl Pill {
    /// The shared pill that draws this one, before its state is set.
    pub fn widget(&self) -> Shown<'_> {
        let p = Shown::new(&self.label);
        let p = self.count.map_or(p, |n| p.count(n as u32));
        self.dot.map_or(p, |k| p.category(k))
    }
}

/// The filter pills of the page the window shows; none for a page without.
pub fn specs(shell: &Shell) -> Vec<Pill> {
    let (labels, on, tally): (&[&str], u8, Option<[usize; 3]>) = match shell.page {
        Page::History => (&FILTERS, shell.hist_filter, None),
        Page::Downloads => (&DOWNLOADS, shell.dl_filter, Some(counts(&shell.downloads))),
        Page::Tabs => return group_pills::specs(shell),
        _ => return Vec::new(),
    };
    let pill = |(i, &label): (usize, &&str)| Pill {
        label: String::from(label),
        count: tally.map(|c| c[i]),
        on: i == on as usize,
        dot: None,
        dim: false,
        pick: Pick::Filter(i as u8),
    };
    labels.iter().enumerate().map(pill).collect()
}
