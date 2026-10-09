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

// Cannon Point-LP, Linux drivers/pinctrl/intel/pinctrl-cannonlake.c cnllp_communities (Whiskey Lake, Comet Lake-U).
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const CNL_LP: Layout = Layout {
    name: "Cannon Point-LP",
    communities: &[
        Community {
            first: 0,
            groups: &[gpp(0, 24, 0), gpp(25, 50, 32), gpp(51, 58, 64), gpp(59, 67, NOMAP)],
        },
        Community {
            first: 68,
            groups: &[
                gpp(68, 92, 96),
                gpp(93, 116, 128),
                gpp(117, 140, 160),
                gpp(141, 172, 192),
                gpp(173, 180, 224),
            ],
        },
        Community {
            first: 181,
            groups: &[
                gpp(181, 204, 256),
                gpp(205, 228, 288),
                gpp(229, 237, NOMAP),
                gpp(238, 243, NOMAP),
            ],
        },
    ],
};
