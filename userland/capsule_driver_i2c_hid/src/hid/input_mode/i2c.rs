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

//! Feature reports over I2C: GET_REPORT and SET_REPORT through the command
//! and data registers the HID descriptor names.

use super::command::command;
use super::get::get;
use super::port::{FeaturePort, REPORT_MAX};
use super::set::set;

pub struct I2cFeatures<'a> {
    pub port: u32,
    pub addr: u8,
    pub desc: &'a [u8; 30],
}

impl FeaturePort for I2cFeatures<'_> {
    fn get(&self, id: u8, report: &mut [u8; REPORT_MAX]) -> usize {
        command(self.desc, id).map_or(0, |cmd| get(self.port, self.addr, &cmd, id, report))
    }

    fn set(&self, report: &[u8]) -> bool {
        let Some(&id) = report.first() else { return false };
        command(self.desc, id).is_some_and(|cmd| set(self.port, self.addr, &cmd, report))
    }
}
