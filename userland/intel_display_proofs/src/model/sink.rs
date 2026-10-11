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

//! A DisplayPort sink, as the standard has a source see it: a DPCD, an
//! EDID behind I2C over AUX, and a receiver that locks its clock only at
//! a rate and width it can take and with at least the drive it needs,
//! then equalises once the drive is enough for that too. It answers in
//! the AUX bytes of DisplayPort 1.4a section 2.7.

use crate::dp::lane::Drive;
use crate::dp::train::Pattern;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SinkState {
    pub dpcd: Vec<u8>,
    pub edid: Vec<u8>,
    pub i2c_offset: usize,
    /// DEFERs to answer before the next ACK, and NACK everything when set.
    pub defers: u32,
    pub nack: bool,
    /// The fastest rate and widest link this receiver locks at.
    pub lock_mbps: u32,
    pub lock_lanes: u32,
    pub cr_needs: Drive,
    pub eq_needs: Drive,
    /// What the source's transmitter is doing.
    pub link: Option<(u32, u32)>,
    pub pattern: Pattern,
    pub drive: Drive,
    pub transfers: u32,
    pub waited_us: u64,
    pub stops: u32,
}

pub type Sink = Rc<RefCell<SinkState>>;

/// A DisplayPort 1.4 sink: 4 lanes at HBR3, TPS3, enhanced framing,
/// 400 us equalisation interval.
pub fn sink(edid: Vec<u8>) -> Sink {
    let mut dpcd = vec![0u8; 0x800];
    dpcd[..4].copy_from_slice(&[0x14, 0x1E, 0x80 | 0x40 | 4, 0x01]);
    Rc::new(RefCell::new(SinkState {
        dpcd,
        edid,
        i2c_offset: 0,
        defers: 0,
        nack: false,
        lock_mbps: 8100,
        lock_lanes: 4,
        cr_needs: Drive { swing: 1, pre: 0 },
        eq_needs: Drive { swing: 1, pre: 1 },
        link: None,
        pattern: Pattern::Off,
        drive: Drive::default(),
        transfers: 0,
        waited_us: 0,
        stops: 0,
    }))
}
