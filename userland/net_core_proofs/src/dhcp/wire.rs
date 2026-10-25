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

//! A link that keeps every frame sent, and the first DHCP message net.core's
//! client puts on it.

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{
    DhcpPacket, EthernetAddress, EthernetFrame, EthernetProtocol, IpProtocol, Ipv4Packet, UdpPacket,
};

use super::create::create;
use super::device::Wire;

const MAC: [u8; 6] = [0x4a, 0x4f, 0xb4, 0x97, 0x46, 0xde];

/// The option codes, in order, and option 55's bytes, of the first DHCP
/// message net.core's client sends.
pub(super) fn first_discover() -> (Vec<u8>, Vec<u8>) {
    let mut wire = Wire::default();
    let mut iface =
        Interface::new(Config::new(EthernetAddress(MAC).into()), &mut wire, Instant::from_secs(0));
    let mut sockets = SocketSet::new(vec![]);
    create(&mut sockets);
    for ms in 0..2000 {
        iface.poll(Instant::from_millis(ms), &mut wire, &mut sockets);
        if !wire.0.is_empty() {
            break;
        }
    }
    let frame = wire.0.first().expect("a DHCPDISCOVER was sent");
    let eth = EthernetFrame::new_checked(&frame[..]).expect("ethernet");
    assert_eq!(eth.ethertype(), EthernetProtocol::Ipv4);
    let ip = Ipv4Packet::new_checked(eth.payload()).expect("ipv4");
    assert_eq!(ip.next_header(), IpProtocol::Udp);
    let udp = UdpPacket::new_checked(ip.payload()).expect("udp");
    assert_eq!((udp.src_port(), udp.dst_port()), (68, 67));
    let dhcp = DhcpPacket::new_checked(udp.payload()).expect("dhcp");
    let mut codes = Vec::new();
    let mut list = Vec::new();
    for o in dhcp.options() {
        codes.push(o.kind);
        if o.kind == 55 {
            list = o.data.to_vec();
        }
    }
    (codes, list)
}
