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

//! The constant relations the bounds stand on. If someone widens the MTU,
//! shrinks a buffer or the reply, this fails before the driver ships.

use crate::constants::{ETH_HEADER_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME, RX_BUFFER_LEN};
use crate::constants::{TX_BUFFER_LEN, TX_QUEUE_SIZE, VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1};
use crate::protocol::{BATCH_MAX_BYTES, HDR_LEN, REPLY_MAX, REQUEST_MAX, RX_FRAME_AT};
use crate::protocol::{RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN, STATUS_LEN};

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn every_frame_handed_either_way_fits_where_it_goes() {
    assert_eq!((VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1), (10, 12));
    for hdr in [VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1] {
        assert!(hdr + MAX_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
        assert!(hdr + MIN_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
        assert!(hdr + MAX_ETHERNET_FRAME <= RX_BUFFER_LEN as usize);
    }
    assert!(ETH_HEADER_LEN <= MIN_ETHERNET_FRAME && MIN_ETHERNET_FRAME <= MAX_ETHERNET_FRAME);
    assert_eq!(RX_FRAME_AT, RESP_HDR_LEN + STATUS_LEN + RX_PAYLOAD_PREFIX_LEN);
    assert_eq!(REPLY_MAX, RX_FRAME_AT + MAX_ETHERNET_FRAME);
    assert_eq!(REQUEST_MAX, HDR_LEN + MAX_ETHERNET_FRAME);
    assert!(4 + RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME <= BATCH_MAX_BYTES);
    assert!(u32::from(TX_QUEUE_SIZE) <= u32::BITS, "one in-flight bit per buffer");
}
