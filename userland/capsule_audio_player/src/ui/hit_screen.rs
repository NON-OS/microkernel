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

//! What a click on each control asks for. A groove's value is read from
//! where along the groove the painter drew the click fell.

use super::control::Control;
use super::geometry::{page, permille, Rect, Shell};
use super::hit::{Action, Lists};
use super::screen::settings::volume_rect;
use super::shell as chrome;
use super::shell::rail_geom::{seek_rect, vol_rect};
use super::state::{UiState, View};
use super::target::{Key, Target};

fn control(k: Key, seek: Rect, volume: Rect, x: i32) -> Control {
    match k {
        Key::Play => Control::PlayPause,
        Key::Prev => Control::Prev,
        Key::Next => Control::Next,
        Key::Shuffle => Control::Shuffle,
        Key::Repeat => Control::Repeat,
        Key::Mute => Control::Mute,
        Key::Seek => Control::Seek(permille(seek, x)),
        Key::Volume => Control::Volume(permille(volume, x)),
    }
}

/// What a click at `(x, y)` on `t` asks for. A queue row means the track it
/// shows, not the row number: under shuffle the queue's order is not the
/// library's.
pub fn action(t: Target, ui: &UiState, sh: &Shell, l: &Lists, x: i32, y: i32) -> Option<Action> {
    let ctl = |c| Some(Action::Ctl(c));
    let (bar, rail) = (chrome::bar(sh.transport), sh.rail);
    match t {
        Target::Bar(k) => ctl(control(k, bar.scrub, bar.volume, x)),
        Target::Rail(k) => ctl(control(k, seek_rect(rail), vol_rect(rail), x)),
        Target::Queue(_) => chrome::rail_queue_track_at(rail, l.queue, x, y).map(Action::Select),
        Target::Nav(v) => Some(Action::Go(v)),
        Target::Field => Some(Action::Go(View::Search)),
        Target::Clear | Target::PageClear => Some(Action::ClearQuery),
        Target::Hero(0) if l.n == 0 => Some(Action::Go(View::Search)),
        Target::Hero(_) if l.n == 0 => Some(Action::Go(View::Downloads)),
        Target::Hero(0) => Some(Action::Select(0)),
        Target::Hero(_) => ctl(Control::Shuffle),
        Target::SeeAll(_) => Some(Action::Go(View::Library)),
        Target::Tile(i) => Some(Action::Select(i)),
        Target::Tab(i) => Some(Action::LibTab(i)),
        Target::Row(i) => l.rows.get(i).copied().map(Action::Select),
        Target::PagePlay if crate::fetch::is_address(&ui.query) => Some(Action::Download),
        Target::PagePlay => l.rows.first().copied().map(Action::Select),
        Target::Toggle(0) => ctl(Control::Shuffle),
        Target::Toggle(1) => ctl(Control::Repeat),
        Target::Toggle(_) => ctl(Control::Mute),
        Target::Volume => ctl(Control::Volume(permille(volume_rect(page(sh)), x))),
        Target::ClearFinished => Some(Action::ClearDownloads),
        Target::Fetch(n, k) => {
            let row = l.downloads.get(n)?;
            row.acts().get(k).map(|&a| Action::Fetched(row.id, a))
        }
    }
}
