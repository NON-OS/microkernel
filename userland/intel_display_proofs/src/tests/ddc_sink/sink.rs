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

//! A hostile HDMI or DVI sink on GMBUS, written from the test's own reading
//! of the controller, not from the driver: an EDID behind 0x50 with E-DDC's
//! segment pointer at 0x30, a DDC/CI responder at 0x37, lies as told (in
//! `lies.rs`), and every try from pin select to stop logged as it saw it.

pub use super::lies::{Eddc, Fault};
use crate::model::Ticks;

#[derive(Clone, Debug, Default)]
pub struct Try {
    /// The segment written, and whether it was acknowledged.
    pub segment: Option<(u8, bool)>,
    /// The read asked for: address, index, length.
    pub read: Option<(u32, u32, usize)>,
    pub sent: Vec<u8>,
    /// Every word the driver took, as bytes.
    pub got: Vec<u8>,
    pub phase: bool,
    /// The clock when the wait phase was shown.
    pub phase_at: u64,
    pub nak: bool,
    pub fault: Fault,
    pub t0: u64,
    pub t1: u64,
}

pub type Answer<'a> = Box<dyn FnMut(&[u8]) -> Vec<u8> + 'a>;

pub struct Sink<'a> {
    pub clock: &'a Ticks,
    pub pins: u32,
    pub edid: Vec<u8>,
    pub eddc: Eddc,
    pub ignore_index: bool,
    /// One per try, in order; None past the end.
    pub faults: Vec<Fault>,
    pub ci: Option<Answer<'a>>,
    pub ci_nak: bool,
    pub log: Vec<Try>,
    pub(super) bus: super::bus::Bus,
}

impl<'a> Sink<'a> {
    /// An honest sink on DDI B's pins holding `edid`, with no DDC/CI.
    pub fn new(clock: &'a Ticks, edid: Vec<u8>) -> Sink<'a> {
        Sink {
            clock,
            pins: 5,
            edid,
            eddc: Eddc::Honest,
            ignore_index: false,
            faults: Vec::new(),
            ci: None,
            ci_nak: false,
            log: Vec::new(),
            bus: Default::default(),
        }
    }
}
