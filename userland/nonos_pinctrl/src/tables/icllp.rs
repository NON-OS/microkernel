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

// Ice Lake-LP, Linux drivers/pinctrl/intel/pinctrl-icelake.c icllp_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout, NOMAP};

pub const ICL_LP: Layout = Layout {
    name: "Ice Lake-LP",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 7, 0), gpp(8, 33, 32), gpp(34, 58, 64)] },
        Community {
            first: 59,
            groups: &[gpp(59, 82, 96), gpp(83, 103, 128), gpp(104, 123, 160), gpp(124, 152, 192)],
        },
        Community {
            first: 153,
            groups: &[
                gpp(153, 176, 224),
                gpp(177, 182, NOMAP),
                gpp(183, 206, 256),
                gpp(207, 215, NOMAP),
            ],
        },
        Community {
            first: 216,
            groups: &[gpp(216, 223, 288), gpp(224, 231, 320), gpp(232, 240, NOMAP)],
        },
    ],
};
