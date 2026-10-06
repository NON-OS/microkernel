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

// Tiger Lake-LP, Linux drivers/pinctrl/intel/pinctrl-tigerlake.c tgllp_communities, which Linux also
// binds to INTC1055 (Alder Lake-P, Raptor Lake-P).
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const TGL_LP: Layout = Layout {
    name: "Tiger Lake-LP",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 25, 0), gpp(26, 41, 32), gpp(42, 66, 64)] },
        Community {
            first: 67,
            groups: &[
                gpp(67, 74, 96),
                gpp(75, 98, 128),
                gpp(99, 119, 160),
                gpp(120, 143, 192),
                gpp(144, 170, 224),
            ],
        },
        Community {
            first: 171,
            groups: &[
                gpp(171, 194, 256),
                gpp(195, 219, 288),
                gpp(220, 225, NOMAP),
                gpp(226, 250, 320),
                gpp(251, 259, NOMAP),
            ],
        },
        Community { first: 260, groups: &[gpp(260, 267, 352), gpp(268, 276, NOMAP)] },
    ],
};
