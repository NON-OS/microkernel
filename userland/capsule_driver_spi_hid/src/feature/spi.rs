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

//! Feature reports over SPI: a get-feature request, answered by the
//! device's get-feature response; a set-feature request, which the device
//! acknowledges. A report's id rides in the body's id byte, its content
//! after.

use nonos_spi_hid::{IN_GET_FEATURE, IN_SET_FEATURE, OUT_GET_FEATURE, OUT_SET_FEATURE};

use super::port::{FeaturePort, REPORT_MAX};
use crate::link::Link;
use crate::report::{ask, await_kind};

pub struct SpiFeatures<'a> {
    pub link: &'a Link,
}

impl FeaturePort for SpiFeatures<'_> {
    fn get(&self, id: u8, report: &mut [u8; REPORT_MAX]) -> usize {
        if !ask(self.link, OUT_GET_FEATURE, id, &[]) {
            return 0;
        }
        let Some(r) = await_kind(self.link, IN_GET_FEATURE).filter(|r| r.id == id) else {
            return 0;
        };
        let n = r.content.len();
        if n == 0 || n >= REPORT_MAX {
            return 0;
        }
        report[0] = id;
        report[1..=n].copy_from_slice(&r.content);
        n + 1
    }

    fn set(&self, report: &[u8]) -> bool {
        let Some((&id, content)) = report.split_first() else { return false };
        ask(self.link, OUT_SET_FEATURE, id, content)
            && await_kind(self.link, IN_SET_FEATURE).is_some_and(|r| r.id == id)
    }
}
