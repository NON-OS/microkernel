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

// Tiger Lake-H, Linux drivers/pinctrl/intel/pinctrl-tigerlake.c tglh_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const TGL_H: Layout = Layout {
    name: "Tiger Lake-H",
    communities: &[
        Community {
            first: 0,
            groups: &[gpp(0, 24, 0), gpp(25, 44, 32), gpp(45, 70, 64), gpp(71, 78, 96)],
        },
        Community {
            first: 79,
            groups: &[
                gpp(79, 104, 128),
                gpp(105, 128, 160),
                gpp(129, 136, 192),
                gpp(137, 153, 224),
                gpp(154, 180, 256),
            ],
        },
        Community { first: 181, groups: &[gpp(181, 193, 288), gpp(194, 217, 320)] },
        Community {
            first: 218,
            groups: &[gpp(218, 241, 352), gpp(242, 251, 384), gpp(252, 266, 416)],
        },
        Community { first: 267, groups: &[gpp(267, 281, 448), gpp(282, 290, NOMAP)] },
    ],
};
