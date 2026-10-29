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

//! Every part of the power-only rule, by its place.

use super::rule::Part;

/// Each part's place, matched whole: a part added to `Part` must come here.
pub fn index(p: Part) -> usize {
    use Part::*;
    match p {
        Pic => 0,
        Osc => 1,
        Sleep => 2,
        Reserved => 3,
        Hidden => 4,
        Hotplug => 5,
        Platform => 6,
        BatteryTrip => 7,
        PowerOff => 8,
        FixedButton => 9,
        Cooling => 10,
        Cpu => 11,
        Backlight => 12,
        HwErrors => 13,
        Ports => 14,
        TypeC => 15,
        DevicePower => 16,
    }
}
