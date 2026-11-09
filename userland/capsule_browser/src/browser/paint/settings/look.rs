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

use crate::browser::shell::kept::MAX_SIZE;
use crate::browser::ui::settings_geom::Control;

/// What the page's buttons show: the appearance, network and reader
/// choices in force, whether a custom search is in use, the text size,
/// whether there is a proxy, history, a finished download or bookmarks to
/// act on, and whether Remove all bookmarks waits for its second click.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shows {
    pub appearance: u8,
    pub network: Option<u8>,
    pub custom: bool,
    pub theme: u8,
    pub width: u8,
    pub size: u8,
    pub proxy: bool,
    pub history: bool,
    pub downloads: bool,
    pub bookmarks: usize,
    pub armed: bool,
}

/* How a button is drawn: the choice in force, another choice, a plain
 * button, one that removes, that one waiting for its second click, or one
 * with nothing to act on. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Look {
    On,
    Off,
    Plain,
    Danger,
    Armed,
    Dim,
}

pub fn look(s: &Shows, c: Control) -> Look {
    let pick = |on: bool| if on { Look::On } else { Look::Off };
    let can = |on: bool| if on { Look::Plain } else { Look::Dim };
    match c {
        Control::Appearance(i) => pick(i == s.appearance),
        Control::Network(i) => pick(Some(i) == s.network),
        Control::Search(i) => pick((i == 1) == s.custom),
        Control::ReaderTheme(i) => pick(i == s.theme),
        Control::ReaderWidth(i) => pick(i == s.width),
        Control::Smaller => can(s.size > 0),
        Control::Larger => can(s.size < MAX_SIZE),
        Control::ProxyOff => can(s.proxy),
        Control::ClearHistory => can(s.history),
        Control::ClearDownloads => can(s.downloads),
        Control::RemoveBookmarks if s.bookmarks == 0 => Look::Dim,
        Control::RemoveBookmarks if s.armed => Look::Armed,
        Control::RemoveBookmarks => Look::Danger,
        Control::ProxySet | Control::SearchUse | Control::Field(_) => Look::Plain,
    }
}
