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

//! The one table of where each control sits, in the order a click has always
//! tried them, read from the geometry the painters draw with.

extern crate alloc;

use alloc::vec::Vec;

use super::geometry::{page, shell, Rect};
use super::shell::bar::bar;
use super::shell::chrome_geom::{clear, field, nav_row, settings_row};
use super::shell::rail_geom::{play_rect, queue_row, queue_visible, seek_rect};
use super::shell::rail_geom::{small_rects, speaker_rect, vol_rect};
use super::shown::Shown;
use super::target::Key::{Mute, Next, Play, Prev, Repeat, Seek, Shuffle, Volume};
use super::target::Target::{self, Bar, Nav, Queue, Rail};
use super::targets_page::page_targets;
use super::view::{View, NAV};

/// Every control on show and the box it is drawn in, but the Library's tabs,
/// which the shared strip places itself.
pub fn targets(s: &Shown) -> Vec<(Target, Rect)> {
    let sh = shell(s.dims.0, s.dims.1);
    let b = bar(sh.transport);
    let mut out = Vec::from([(Bar(Play), b.play), (Bar(Prev), b.prev), (Bar(Next), b.next)]);
    out.extend([(Bar(Mute), b.speaker), (Bar(Shuffle), b.shuffle), (Bar(Repeat), b.repeat)]);
    out.extend([(Bar(Seek), b.scrub), (Bar(Volume), b.volume)]);
    out.push((Nav(View::Settings), settings_row(sh.sidebar)));
    out.extend(NAV.iter().enumerate().map(|(i, &(v, _))| (Nav(v), nav_row(sh.sidebar, i))));
    if sh.rail.w > 0 {
        let (r, [prev, next, shuffle, repeat]) = (sh.rail, small_rects(sh.rail));
        out.extend([(Rail(Play), play_rect(r)), (Rail(Prev), prev), (Rail(Next), next)]);
        out.extend([(Rail(Shuffle), shuffle), (Rail(Repeat), repeat)]);
        out.extend([(Rail(Mute), speaker_rect(r)), (Rail(Seek), seek_rect(r))]);
        out.push((Rail(Volume), vol_rect(r)));
        out.extend((0..queue_visible(r).min(s.queue)).map(|i| (Queue(i), queue_row(r, i))));
    }
    if s.query {
        out.push((Target::Clear, clear(sh.topbar)));
    }
    out.push((Target::Field, field(sh.topbar)));
    page_targets(&mut out, s, page(&sh));
    out
}
