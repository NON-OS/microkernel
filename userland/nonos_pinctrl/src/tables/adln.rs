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

// Alder Lake-N, Linux drivers/pinctrl/intel/pinctrl-alderlake.c adln_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const ADL_N: Layout = Layout {
    name: "Alder Lake-N",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 25, 0), gpp(26, 41, 32), gpp(42, 66, 64)] },
        Community {
            first: 67,
            groups: &[
                gpp(67, 74, 96),
                gpp(75, 94, 128),
                gpp(95, 118, 160),
                gpp(119, 139, 192),
                gpp(140, 168, 224),
            ],
        },
        Community {
            first: 169,
            groups: &[
                gpp(169, 192, 256),
                gpp(193, 217, 288),
                gpp(218, 223, NOMAP),
                gpp(224, 248, 320),
            ],
        },
        Community { first: 249, groups: &[gpp(249, 256, 352)] },
    ],
};
