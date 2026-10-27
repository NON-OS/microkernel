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

//! What a guest's connect is told when its Nym stream did not open, read
//! from route link's own sentences: the destination refusing is never told
//! as the network, and the network is never told as the destination.

use crate::linux::abi::errno::{ECONNREFUSED, EHOSTUNREACH, ENETUNREACH, ETIMEDOUT};
use crate::linux::net::routed::nym_errno::errno_of;
use crate::linux::net::routed::socks::reply::{code_of, CONNECTION_REFUSED, NETWORK_UNREACHABLE};
use crate::refusal::Proxy;

#[test]
fn every_reply_code_net_socks5_can_give_has_its_errno() {
    for rep in 0u8..=255 {
        let want = match rep {
            2 | 5 => ECONNREFUSED,
            4 => EHOSTUNREACH,
            6 => ETIMEDOUT,
            _ => ENETUNREACH,
        };
        assert_eq!(errno_of(Proxy::Nym.refused(rep)), want, "reply {rep}");
    }
}

#[test]
fn the_proxy_failing_is_no_route_out() {
    let nym = Proxy::Nym;
    for why in [nym.slow(), nym.silent(), nym.full(), nym.garbled(), nym.lost(), nym.ended()] {
        assert_eq!(errno_of(why), ENETUNREACH, "{why}");
        assert_eq!(code_of(errno_of(why)), NETWORK_UNREACHABLE);
    }
    /* "not connected yet", what is left after the 45 s, is the network too. */
    assert_eq!(errno_of(nym.refused(3)), ENETUNREACH);
    assert_eq!(code_of(errno_of(nym.refused(5))), CONNECTION_REFUSED);
}

#[test]
fn another_network_s_sentence_is_not_read_as_nym_s() {
    for rep in [2u8, 4, 5, 6] {
        assert_eq!(errno_of(Proxy::Anyone.refused(rep)), ENETUNREACH, "Anyone's {rep}");
    }
}
