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

//! K3: a device a unit confines can still write 0xFEEx_xxxx, so its claim
//! needs interrupt remapping, or the opt-in outside a production build.

use super::open::interrupts_confined;

#[test]
fn a_claim_needs_remapping_or_the_development_opt_in() {
    // (remapping, opt_in, production) -> claimable
    let cases = [
        ((true, false, false), true),
        ((true, false, true), true),
        ((true, true, true), true),
        ((false, false, false), false),
        ((false, false, true), false),
        ((false, true, false), true),
        ((false, true, true), false),
    ];
    for ((remapping, opt_in, production), want) in cases {
        let got = interrupts_confined(remapping, opt_in, production, false);
        assert_eq!(got, want, "remapping={remapping} opt_in={opt_in} production={production}");
    }
}
