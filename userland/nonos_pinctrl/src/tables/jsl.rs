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

// Jasper Lake, Linux drivers/pinctrl/intel/pinctrl-jasperlake.c jsl_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP, ZERO};

pub const JSL: Layout = Layout {
    name: "Jasper Lake",
    communities: &[
        Community {
            first: 0,
            groups: &[
                gpp(0, 19, 320),
                gpp(20, 28, NOMAP),
                gpp(29, 54, 32),
                gpp(55, 75, 64),
                gpp(76, 83, 96),
                gpp(84, 91, 128),
            ],
        },
        Community {
            first: 92,
            groups: &[
                gpp(92, 115, 160),
                gpp(116, 141, 192),
                gpp(142, 170, 224),
                gpp(171, 194, 256),
            ],
        },
        Community { first: 195, groups: &[gpp(195, 200, NOMAP), gpp(201, 224, 288)] },
        Community { first: 225, groups: &[gpp(225, 232, ZERO)] },
    ],
};
