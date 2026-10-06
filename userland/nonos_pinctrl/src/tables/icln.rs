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

// Ice Lake-N, Linux drivers/pinctrl/intel/pinctrl-icelake.c icln_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP, ZERO};

pub const ICL_N: Layout = Layout {
    name: "Ice Lake-N",
    communities: &[
        Community {
            first: 0,
            groups: &[
                gpp(0, 8, NOMAP),
                gpp(9, 34, 32),
                gpp(35, 55, 64),
                gpp(56, 63, 96),
                gpp(64, 71, 128),
            ],
        },
        Community {
            first: 72,
            groups: &[gpp(72, 95, 160), gpp(96, 121, 192), gpp(122, 150, 224), gpp(151, 174, 256)],
        },
        Community { first: 175, groups: &[gpp(175, 180, NOMAP), gpp(181, 204, 288)] },
        Community { first: 205, groups: &[gpp(205, 212, ZERO)] },
    ],
};
