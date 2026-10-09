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

// Cannon Lake-H, Linux drivers/pinctrl/intel/pinctrl-cannonlake.c cnlh_communities (Coffee Lake-H, Comet Lake-H).
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const CNL_H: Layout = Layout {
    name: "Cannon Lake-H",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 24, 0), gpp(25, 50, 32)] },
        Community {
            first: 51,
            groups: &[
                gpp(51, 74, 64),
                gpp(75, 98, 96),
                gpp(99, 106, 128),
                gpp(107, 114, NOMAP),
                gpp(115, 146, 160),
                gpp(147, 154, NOMAP),
            ],
        },
        Community {
            first: 155,
            groups: &[
                gpp(155, 178, 192),
                gpp(179, 202, 224),
                gpp(203, 215, 256),
                gpp(216, 239, 288),
                gpp(240, 248, NOMAP),
            ],
        },
        Community {
            first: 249,
            groups: &[
                gpp(249, 259, NOMAP),
                gpp(260, 268, NOMAP),
                gpp(269, 286, 320),
                gpp(287, 298, 352),
            ],
        },
    ],
};
