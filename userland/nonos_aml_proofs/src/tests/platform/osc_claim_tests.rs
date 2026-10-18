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

//! _OSC asked for every control and committed with only the claim: what
//! firmware would give up is kept, and nothing past the claim is taken.

use super::osc_tests::osc_for;
use super::run::evaluator;
use crate::consts::at;
use crate::platform::{offer_then_claim, PCI_HOST_UUID};
use crate::probe::Probe;
use crate::run::{boot, num};

#[test]
fn offered_is_asked_and_only_the_claim_is_committed() {
    let (mut aml, _) = boot(&osc_for(PCI_HOST_UUID[0]), 2);
    let osc = at(&aml, "\\_SB.PCI0._OSC").unwrap();
    let mut probe = Probe::default();
    let mut eval = evaluator(&mut probe);
    let host = (&PCI_HOST_UUID, 1);
    for (claim, granted, committed) in [(0, 0, 0), (0x01, 0x01, 0x01), (0x08, 0, 0)] {
        let (o, g) =
            offer_then_claim(&mut aml, &mut eval, osc, host, &[0, 0x11, 0xBD], &[0, !0, claim])
                .unwrap();
        assert_eq!((o[2], g[2]), (0x01, granted), "claim {claim:#x}");
        let id = at(&aml, "\\LCT").unwrap();
        assert_eq!(num(&eval(&mut aml, id, vec![]).unwrap()), committed, "claim {claim:#x}");
    }
}
