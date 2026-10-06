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

// Alder Lake-S, Linux drivers/pinctrl/intel/pinctrl-alderlake.c adls_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const ADL_S: Layout = Layout {
    name: "Alder Lake-S",
    communities: &[
        Community {
            first: 0,
            groups: &[
                gpp(0, 24, 0),
                gpp(25, 47, 32),
                gpp(48, 59, 64),
                gpp(60, 86, 96),
                gpp(87, 94, 128),
            ],
        },
        Community {
            first: 95,
            groups: &[gpp(95, 118, 160), gpp(119, 126, 192), gpp(127, 150, 224)],
        },
        Community {
            first: 151,
            groups: &[gpp(151, 159, NOMAP), gpp(160, 175, 256), gpp(176, 199, 288)],
        },
        Community {
            first: 200,
            groups: &[
                gpp(200, 207, 320),
                gpp(208, 230, 352),
                gpp(231, 245, 384),
                gpp(246, 269, 416),
            ],
        },
        Community { first: 270, groups: &[gpp(270, 294, 448), gpp(295, 303, NOMAP)] },
    ],
};
