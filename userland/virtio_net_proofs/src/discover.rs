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

//! How many device records discovery reads, as it ships.

#[path = "../../capsule_driver_virtio_net/src/discover/constants.rs"]
mod constants;

#[cfg(test)]
mod tests {
    use super::constants::MAX_DEVICES;

    // At 32 records a machine with more stopped short of a device behind a
    // root port; every wired driver reads 128.
    #[test]
    #[allow(clippy::assertions_on_constants)] // the shipped bound is the point
    fn discovery_reads_as_many_records_as_the_wired_drivers() {
        assert!(MAX_DEVICES >= 128, "discovery reads {MAX_DEVICES} records");
    }
}
