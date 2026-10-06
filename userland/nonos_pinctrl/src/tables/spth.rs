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

// Sunrise Point-H, Linux drivers/pinctrl/intel/pinctrl-sunrisepoint.c spth_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout};

pub const SPT_H: Layout = Layout {
    name: "Sunrise Point-H",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 23, 0), gpp(24, 47, 24)] },
        Community {
            first: 48,
            groups: &[
                gpp(48, 71, 48),
                gpp(72, 95, 72),
                gpp(96, 108, 96),
                gpp(109, 132, 120),
                gpp(133, 156, 144),
                gpp(157, 180, 168),
            ],
        },
        Community { first: 181, groups: &[gpp(181, 191, 192)] },
    ],
};
