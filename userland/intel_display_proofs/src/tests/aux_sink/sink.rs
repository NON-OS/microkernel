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

//! A hostile DisplayPort sink's I2C-over-AUX end, written from the test's
//! own reading of AUX framing (DisplayPort 1.4a section 2.7) and E-DDC,
//! not from the driver: an EDID behind 0x50 and the segment pointer at
//! 0x30. It lies about the pointer, the index or one try's bytes (G-69's
//! lies), and about each transfer's AUX answer at G-66's kinds of rates.
//! Each I2C transaction, from its first request to its stop, is logged as
//! the sink saw it.

pub use super::aux_log::{Rates, Try};
pub use crate::tests::ddc_sink::lies::{Eddc, Fault};

pub struct AuxSink {
    pub edid: Vec<u8>,
    pub eddc: Eddc,
    pub ignore_index: bool,
    pub faults: Vec<Fault>,
    pub rates: Rates,
    pub seed: u64,
    pub log: Vec<Try>,
    pub transfers: u32,
    pub(super) open: bool,
    pub(super) stopping: bool,
    pub(super) segment: u8,
    pub(super) stream: Vec<u8>,
    pub(super) pos: usize,
}

impl AuxSink {
    pub fn new(edid: Vec<u8>) -> AuxSink {
        let (eddc, ignore_index, faults, rates) =
            (Eddc::Honest, false, Vec::new(), Rates::default());
        let (open, stopping, segment, stream, pos) = (false, false, 0, Vec::new(), 0);
        AuxSink {
            edid,
            eddc,
            ignore_index,
            faults,
            rates,
            seed: 1,
            log: Vec::new(),
            transfers: 0,
            open,
            stopping,
            segment,
            stream,
            pos,
        }
    }

    pub(super) fn roll(&mut self, n: u32) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        (self.seed % n.max(1) as u64) as u32
    }
}
