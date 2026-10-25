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

//! What the driver counts, for OP_STATS and for noticing a device gone.

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Stats {
    pub tx_frames: u32,
    pub tx_errors: u32,
    pub rx_frames: u32,
    pub rx_errors: u32,
    pub tx_bytes: u32,
    pub rx_bytes: u32,
    /// Devices bound, and devices given up as gone, since start.
    pub binds: u32,
    pub losses: u32,
    /// Failures since a frame last moved, transfers and the driver's own
    /// looks alike: an empty IN pipe cannot hide a device whose sends fail.
    pub failing: u32,
    /// Where the station address came from (STATION_*), for Settings'
    /// network page: a driver that never has to fall back leaves it 0.
    pub station: u32,
}

/// The station address's source, word 9 of OP_STATS: not reported (every
/// driver that always serves its drawn address); the drawn address, the
/// network having answered it; the host-assigned address, nothing having
/// answered the drawn one in the bounded wait (Hyper-V with MAC spoofing
/// off); the host-assigned address, the host having refused the drawn one.
pub const STATION_UNREPORTED: u32 = 0;
pub const STATION_DRAWN: u32 = 1;
pub const STATION_HOST_UNANSWERED: u32 = 2;
pub const STATION_HOST_REFUSED: u32 = 3;

impl Stats {
    pub fn tx(&mut self, sent: Result<usize, i32>) {
        let Ok(n) = sent else { return self.failed(true) };
        self.tx_frames = self.tx_frames.wrapping_add(1);
        self.tx_bytes = self.tx_bytes.wrapping_add(n as u32);
        self.failing = 0;
    }

    pub fn rx(&mut self, got: Result<Option<usize>, i32>) {
        let Ok(got) = got else { return self.failed(false) };
        if let Some(n) = got {
            self.rx_frames = self.rx_frames.wrapping_add(1);
            self.rx_bytes = self.rx_bytes.wrapping_add(n as u32);
            self.failing = 0;
        }
    }

    /// The driver's own look (`Nic::tick`): a failure counts toward the
    /// device being gone, and a success moves no frame.
    pub fn ticked(&mut self, r: Result<(), i32>) {
        self.failing = self.failing.saturating_add(r.is_err() as u32);
    }

    fn failed(&mut self, tx: bool) {
        let count = if tx { &mut self.tx_errors } else { &mut self.rx_errors };
        *count = count.wrapping_add(1);
        self.failing = self.failing.saturating_add(1);
    }
}
