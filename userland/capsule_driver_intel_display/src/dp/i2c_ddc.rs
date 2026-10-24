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

//! DDC/CI over the AUX channel, for a DisplayPort monitor: I2C over AUX
//! (DisplayPort 1.4a section 2.7.7.1.6) to the DDC/CI address only. A write
//! goes in one transaction with middle-of-transaction set, a read in as
//! many as sixteen bytes allow, and each ends with an address-only request
//! without it, so the sink's I2C bus sees a stop.

use super::aux::{header, Aux, AuxError, I2C_MOT, I2C_READ, I2C_WRITE, MAX_DATA};
use super::i2c::once;
use crate::ddc::bus::DdcBus;
use crate::ddc::error::DdcError;
use crate::ddc::frame::DDC_CI_ADDR;

pub struct AuxDdc<'a> {
    pub aux: &'a mut dyn Aux,
}

impl DdcBus for AuxDdc<'_> {
    fn send(&mut self, bytes: &[u8]) -> Result<(), DdcError> {
        if bytes.is_empty() || bytes.len() > MAX_DATA {
            return Err(DdcError::Bus);
        }
        let mut w = [0u8; 4 + MAX_DATA];
        w[..4].copy_from_slice(&header(I2C_WRITE | I2C_MOT, DDC_CI_ADDR, bytes.len()));
        w[4..4 + bytes.len()].copy_from_slice(bytes);
        let r = once(self.aux, &w[..4 + bytes.len()]).map(|_| ());
        let stop = once(self.aux, &header(I2C_WRITE, DDC_CI_ADDR, 1)[..3]).map(|_| ());
        r.and(stop).map_err(|_| DdcError::Bus)
    }

    fn receive(&mut self, out: &mut [u8]) -> Result<(), DdcError> {
        let r = take(self.aux, out);
        let stop = once(self.aux, &header(I2C_READ, DDC_CI_ADDR, 1)[..3]);
        r.and(stop.map(|_| ())).map_err(|_| DdcError::Bus)
    }
}

/// The reply's bytes, as many as each transaction gives; one that gives
/// none is short, so a byte is never left unread.
fn take(aux: &mut dyn Aux, out: &mut [u8]) -> Result<(), AuxError> {
    let mut done = 0;
    while done < out.len() {
        let n = (out.len() - done).min(MAX_DATA);
        let (b, len) = once(aux, &header(I2C_READ | I2C_MOT, DDC_CI_ADDR, n))?;
        let got = len.saturating_sub(1).min(n);
        if got == 0 {
            return Err(AuxError::Short);
        }
        out[done..done + got].copy_from_slice(&b[1..1 + got]);
        done += got;
    }
    Ok(())
}
