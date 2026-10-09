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

// Sunrise Point-LP, Linux drivers/pinctrl/intel/pinctrl-sunrisepoint.c sptlp_communities. Linux sizes
// these communities (SPT_LP_COMMUNITY, 24 pads a group) and numbers a sized
// group's GPIOs from its first pin, which is gpio_base 0 (MATCH) below.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout};

pub const SPT_LP: Layout = Layout {
    name: "Sunrise Point-LP",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 23, 0), gpp(24, 47, 0)] },
        Community { first: 48, groups: &[gpp(48, 71, 0), gpp(72, 95, 0), gpp(96, 119, 0)] },
        Community { first: 120, groups: &[gpp(120, 143, 0), gpp(144, 151, 0)] },
    ],
};
