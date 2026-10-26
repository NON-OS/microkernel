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

//! The stop of a claimed device that has no config space, so no Bus Master
//! to clear, before its frames are freed: a described device's interface's
//! own (`described::stop`), or the reset of an SD host found through ACPI
//! (`dma::stop_sd_host`, F-128). False for any other device.

pub(super) fn stopped(device_id: u64) -> bool {
    crate::hardware::broker::described::stop(device_id)
        || crate::hardware::broker::dma::stop_sd_host(device_id)
}
