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

//! The kind-2 container the job builds is one the buyer's verify reads,
//! record and body accepted, up to the device proof.

use nonos_proof_abi::isolation::STATEMENT;
use nonos_proof_abi::trust::{DeviceRoot, Trust, LINUX_CAPS};
use nonos_proof_abi::{verify_delivery, DeliveryExpected, PolicyView, Refusal};

use crate::fixture::{ask, live, LISTING};
use crate::market::delivery::{body_of, container2};

const DAY: u64 = 20_372;

fn read(p: &[u8]) -> Option<PolicyView> {
    let set = p.get(152..217)?.try_into().ok()?;
    let mask = u64::from_le_bytes(p.get(48..56)?.try_into().ok()?);
    Some(PolicyView {
        program_blake3: p.get(16..48)?.try_into().ok()?,
        mask,
        listing_id: p.get(56..88)?.try_into().ok()?,
        syscall_set: set,
    })
}

#[test]
fn the_container_passes_the_buyers_verify_up_to_the_device_proof() {
    let (s, p) = live();
    let body = body_of(&s, &ask(&p, b"ask", b"answer"));
    let c = container2(DAY, &s, &[0xDE; 40], &body).expect("a kind-2 container");
    let reg = [DeviceRoot { root: [1; 4], depth: 4, from_day: 0, life_days: 99_999 }];
    let sup = [s.record.sup_meas];
    let pins = Trust {
        boot_root: [1; 4],
        kernel_root: [1; 4],
        registrars: &reg,
        supervisors: &sup,
        vendor_policies: &[],
        isolation: &STATEMENT,
    };
    let ex = DeliveryExpected {
        listing_id: LISTING,
        policy: &p,
        nonce: [0x6E; 32],
        request: b"ask",
        response: b"answer",
        order_id: [0x0D; 32],
        channel_id: [0xC4; 32],
        days: (DAY, DAY),
        pins: &pins,
        read_policy: read,
    };
    assert_eq!(s.record.sup_caps, LINUX_CAPS);
    assert_eq!(
        verify_delivery(&c, &ex).err(),
        Some(Refusal::DeviceBinding),
        "record and body accepted"
    );
    let other = DeliveryExpected { response: b"another answer", ..ex };
    assert_eq!(verify_delivery(&c, &other).err(), Some(Refusal::Body));
}
