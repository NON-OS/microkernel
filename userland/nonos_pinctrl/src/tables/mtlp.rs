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

// Meteor Lake-P, Linux drivers/pinctrl/intel/pinctrl-meteorlake.c mtlp_communities.
// Each gpp() is that file's INTEL_GPP(reg, start, end, gpio_base) line.

use crate::group::{gpp, Community, Layout};

pub const MTL_P: Layout = Layout {
    name: "Meteor Lake-P",
    communities: &[
        Community { first: 0, groups: &[gpp(0, 4, 0), gpp(5, 28, 32), gpp(29, 52, 64)] },
        Community { first: 53, groups: &[gpp(53, 77, 96), gpp(78, 102, 128)] },
        Community {
            first: 103,
            groups: &[
                gpp(103, 128, 160),
                gpp(129, 154, 192),
                gpp(155, 169, 224),
                gpp(170, 183, 256),
            ],
        },
        Community { first: 184, groups: &[gpp(184, 191, 288), gpp(192, 203, 320)] },
        Community {
            first: 204,
            groups: &[
                gpp(204, 228, 352),
                gpp(229, 253, 384),
                gpp(254, 285, 416),
                gpp(286, 288, 448),
            ],
        },
    ],
};
