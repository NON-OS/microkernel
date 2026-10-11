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

//! One row of the catalogue.

use alloc::vec::Vec;

use super::market::{Detail, Readiness, Release};
use super::price::Price;

pub use super::source::Source;

pub struct Listing {
    pub id: Vec<u8>,
    pub measurement: [u8; 32],
    pub name: Vec<u8>,
    /// The market capsule's verdict across every install gate, taken as given.
    pub ready: bool,
    pub source: Source,
    /// Where an install of it stands, as the system last said.
    pub progress: super::progress::Progress,
    /// The last thing asked for it was an uninstall, so a failure is the
    /// uninstall's and Retry asks to uninstall again.
    pub removing: bool,
    /// What the market said about it beyond the list, asked once per
    /// catalogue and kept: selecting it again, or typing a search, asks
    /// nothing.
    pub known: Known,
}

/// The answers kept for one listing. Each is asked once; a None after
/// asking is the market's silence, shown as such rather than asked again
/// on every frame.
#[derive(Default)]
pub struct Known {
    /// Publisher and description. Asked for every listing in turn, so a
    /// search can match descriptions.
    pub detail: Option<Detail>,
    pub described: bool,
    /// The default release and the install gates, asked when the listing
    /// is first selected.
    pub release: Option<Release>,
    pub readiness: Option<Readiness>,
    /// What the release sells and how, from OP_GET_COMMERCE (B9).
    pub price: Option<Price>,
    pub judged: bool,
    /// When to ask the gates again while the market reads the bond, and
    /// how many times it has been asked so (fill_order::RECHECKS at most).
    pub recheck_at: Option<u64>,
    pub rechecks: u8,
}

impl Listing {
    pub fn new(id: Vec<u8>, measurement: [u8; 32], name: Vec<u8>, ready: bool) -> Listing {
        let source = Source::of(&id);
        let progress = super::progress::Progress::Idle;
        let known = Known::default();
        Listing { id, measurement, name, ready, source, progress, removing: false, known }
    }
}
