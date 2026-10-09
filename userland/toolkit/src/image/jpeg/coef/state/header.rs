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

use super::State;
use crate::image::jpeg::coef::frame::parse_frame;
use crate::image::jpeg::coef::huff::Coder;
use crate::image::jpeg::coef::refuse::OTHER_PROCESS;
use crate::image::jpeg::dht::parse_dht;
use crate::image::jpeg::dqt::parse_dqt;
use crate::image::types::DecodeError;

impl State {
    /// Apply a table, frame or application segment before a scan.
    pub fn header(&mut self, m: u8, seg: &[u8]) -> Result<(), DecodeError> {
        match m {
            0xC0..=0xC2 => self.frame(parse_frame(seg, m == 0xC2)?),
            0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF => Err(OTHER_PROCESS),
            0xC4 => {
                let [dc, ac] = &mut self.tables;
                parse_dht(seg, dc, ac)?;
                self.dc = dc.iter().map(|t| Coder::new(t.clone())).collect();
                self.ac = ac.iter().map(|t| Coder::new(t.clone())).collect();
                Ok(())
            }
            0xDB => parse_dqt(seg, &mut self.qt),
            0xDD if seg.len() >= 2 => {
                self.ri = u16::from_be_bytes([seg[0], seg[1]]) as usize;
                Ok(())
            }
            0xEE if seg.len() >= 12 && seg.starts_with(b"Adobe") => {
                self.adobe = Some(seg[11]);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
