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

//! The market's OP_GET_LINUX_POLICY reply, as init reads it: neither reader
//! panics, the two agree on every byte string, and whatever they accept is
//! exactly what the market's writer would have sent for it, so no reply is
//! read two ways; and init takes it only for the pin it was made for.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_market_proto::{linux_policy_body, parse_linux_policy, LinuxPolicy};

#[path = "../../../../src/security/market_capsule/client/linux_policy_wire.rs"]
mod kernel;

fuzz_target!(|data: &[u8]| {
    let proto = parse_linux_policy(data);
    let read = kernel::parse(data);
    match (&proto, &read) {
        (None, None) => {}
        (Some((p, a)), Some((k, b))) => {
            assert_eq!(a, b);
            let same = match (p, k) {
                (LinuxPolicy::NoPolicy, kernel::LinuxPolicyReply::NoPolicy) => true,
                (LinuxPolicy::Refused, kernel::LinuxPolicyReply::Refused) => true,
                (LinuxPolicy::Policy(x), kernel::LinuxPolicyReply::Policy(y)) => x == y,
                _ => false,
            };
            assert!(same, "the two readers read one reply two ways");
            assert_eq!(linux_policy_body(p, a), data, "accepted bytes the writer never sends");
            let other = a.map(|x| x ^ 1);
            assert!(kernel::held_to(read.clone().expect("read"), a).is_some(), "its own pin");
            assert!(kernel::held_to(read.clone().expect("read"), &other).is_none(), "another pin");
        }
        _ => panic!("one reader accepts what the other refuses"),
    }
});
