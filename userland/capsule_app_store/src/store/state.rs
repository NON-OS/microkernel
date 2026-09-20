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

//! What the window is showing.

pub use super::tab::{Tab, TABS};

use alloc::vec::Vec;

use super::listing::Listing;
use super::market;


pub struct State {
    pub listings: Vec<Listing>,
    pub tab: Tab,
    pub cursor: usize,
    pub scroll: usize,
    pub rows: usize,
    pub fb_w: u32,
    pub fb_h: u32,
    /// Set when the catalogue could not be read.
    pub trouble: Option<&'static [u8]>,
    pub ready: Option<market::Readiness>,
    /// What the last install request was answered with, shown until the next
    /// one.
    pub asked: Option<super::install::Asked>,
    /// Where enrolment has got to.
    pub consent: super::consent::Consent,
    /// Description and publisher for the selected listing, fetched once per
    /// selection.
    pub search: super::search::Search,
    pub detail: Option<market::Detail>,
}
