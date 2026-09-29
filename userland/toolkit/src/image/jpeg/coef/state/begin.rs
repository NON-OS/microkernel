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
use crate::image::jpeg::coef::frame::Frame;
use crate::image::jpeg::coef::plane::{block_bytes, Plane};
use crate::image::jpeg::coef::refuse::MALFORMED;
use crate::image::jpeg::coef::scan::{parse_scan, Scan};
use crate::image::jpeg::zigzag::ZIGZAG;
use crate::image::types::DecodeError;

impl State {
    pub(super) fn frame(&mut self, f: Frame) -> Result<(), DecodeError> {
        let comps = &f.comps[..f.n];
        let need =
            comps.iter().map(|c| c.bw * c.bh * block_bytes(self.k, f.progressive)).sum::<usize>();
        if self.frame.is_some() || need > self.max_coef {
            return Err(DecodeError::BadDimensions);
        }
        for c in comps {
            self.planes.push(Plane::new(c.bw, c.bh, self.k, f.progressive)?);
        }
        self.frame = Some(f);
        Ok(())
    }

    /// Parse a scan header and fix the quant table of each component it
    /// starts, refusing one that no DQT defined.
    pub fn begin_scan(&mut self, seg: &[u8]) -> Result<Scan, DecodeError> {
        let f = self.frame.as_ref().ok_or(MALFORMED)?;
        let s = parse_scan(seg, f)?;
        for &c in &s.comp[..s.n] {
            let t = &self.qt[f.comps[c].tq];
            if !self.taken[c] {
                if !t.present {
                    return Err(MALFORMED);
                }
                for (z, &v) in t.values.iter().enumerate() {
                    self.qnat[c][ZIGZAG[z]] = v;
                }
                self.taken[c] = true;
            }
        }
        Ok(s)
    }
}
