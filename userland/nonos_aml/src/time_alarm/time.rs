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

//! The 16-byte time _GRT returns and _SRT takes: year u16, month, day,
//! hour, minute, second, a valid byte, milliseconds u16, the time zone as
//! signed minutes from UTC (2047 unspecified), a daylight byte, three pad
//! bytes. A buffer of another length or a field out of its range is no
//! time: firmware's clock is read only when every field holds.

pub const TZ_UNSPECIFIED: i16 = 2047;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millis: u16,
    pub tz_minutes: i16,
    pub daylight: u8,
}

impl RealTime {
    fn sound(&self) -> bool {
        let tz = self.tz_minutes == TZ_UNSPECIFIED || (-1440..=1440).contains(&self.tz_minutes);
        let date = (1..=12).contains(&self.month) && (1..=31).contains(&self.day);
        let clock = self.hour < 24 && self.minute < 60 && self.second < 60 && self.millis < 1000;
        date && clock && tz && self.year >= 1900
    }

    /// None for a buffer that is not 16 bytes, says invalid, or holds a
    /// field out of range.
    pub fn decode(b: &[u8]) -> Option<Self> {
        let b: &[u8; 16] = b.try_into().ok()?;
        let t = RealTime {
            year: u16::from_le_bytes([b[0], b[1]]),
            month: b[2],
            day: b[3],
            hour: b[4],
            minute: b[5],
            second: b[6],
            millis: u16::from_le_bytes([b[8], b[9]]),
            tz_minutes: i16::from_le_bytes([b[10], b[11]]),
            daylight: b[12],
        };
        (b[7] == 1 && t.sound()).then_some(t)
    }

    /// None for a time no clock should be set to.
    pub fn encode(&self) -> Option<[u8; 16]> {
        let [y0, y1] = self.year.to_le_bytes();
        let [m0, m1] = self.millis.to_le_bytes();
        let [z0, z1] = self.tz_minutes.to_le_bytes();
        let b = [y0, y1, self.month, self.day, self.hour, self.minute, self.second, 1, m0, m1];
        let mut out = [0u8; 16];
        out[..10].copy_from_slice(&b);
        out[10..13].copy_from_slice(&[z0, z1, self.daylight]);
        self.sound().then_some(out)
    }
}
