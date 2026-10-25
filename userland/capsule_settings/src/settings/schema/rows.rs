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

use super::access_knob::Knob;
pub use super::bt_live::BtLive;
use nonos_policy_proto::Field;

/// A value the panel reads from the running system, not from policy.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Live {
    LinkState,
    IpAddress,
    Gateway,
    Dns,
    Adapter,
    Version,
    Commit,
    Toolchain,
    Architecture,
    WifiLink,
    WifiJoin,
    WifiRemember,
    MachineKey,
    AudioOutput,
    Bt(BtLive),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Ok,
    Warn,
    Idle,
}

/// The badge on a card header, resolved against live state at paint time.
#[derive(Clone, Copy)]
pub enum Pill {
    None,
    Net,
    Radio,
    Bt,
}

#[derive(Clone, Copy)]
pub enum Row {
    Field(Field),
    Live(&'static str, Live),
    Networks,
    Saved,
    Wallpapers,   // Every wallpaper, a row each.
    Access(Knob), // A row the compositor holds (`access_knob`).
    BtDevices,
    Station, // Where the wired address came from, when its driver says.
}

#[derive(Clone, Copy)]
pub struct Block {
    pub title: &'static str,
    pub note: Option<&'static str>,
    pub pill: Pill,
    pub rows: &'static [Row],
}
