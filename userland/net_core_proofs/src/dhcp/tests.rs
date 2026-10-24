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

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;
use smoltcp::wire::{
    DhcpPacket, EthernetAddress, EthernetFrame, EthernetProtocol, IpProtocol, Ipv4Packet, UdpPacket,
};

use super::create::create;

/// Windows 10 and 11's option 55, in order.
const WINDOWS: [u8; 14] = [1, 3, 6, 15, 31, 33, 43, 44, 46, 47, 119, 121, 249, 252];
/// Options that would name the machine or its software.
const NAMING: [u8; 4] = [12, 60, 77, 81];

#[derive(Default)]
struct Wire(Vec<Vec<u8>>);

struct Rx;
struct Tx<'a>(&'a mut Wire);

impl RxToken for Rx {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, f: F) -> R {
        f(&mut [])
    }
}

impl TxToken for Tx<'_> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut frame = vec![0u8; len];
        let r = f(&mut frame);
        self.0 .0.push(frame);
        r
    }
}

impl Device for Wire {
    type RxToken<'a> = Rx;
    type TxToken<'a> = Tx<'a>;
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1514;
        caps
    }
    fn receive(&mut self, _now: Instant) -> Option<(Rx, Tx<'_>)> {
        None
    }
    fn transmit(&mut self, _now: Instant) -> Option<Tx<'_>> {
        Some(Tx(self))
    }
}

const MAC: [u8; 6] = [0x4a, 0x4f, 0xb4, 0x97, 0x46, 0xde];

/// The option codes, in order, and option 55's bytes, of the first DHCP
/// message net.core's client sends.
fn first_discover() -> (Vec<u8>, Vec<u8>) {
    let mut wire = Wire::default();
    let mut iface = Interface::new(Config::new(EthernetAddress(MAC).into()), &mut wire, Instant::from_secs(0));
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

#[test]
fn the_parameter_request_list_is_windows_in_its_order() {
    let (_, list) = first_discover();
    assert_eq!(list, WINDOWS, "option 55 on the wire");
}

#[test]
fn nothing_that_names_the_machine_is_sent() {
    let (codes, _) = first_discover();
    for c in NAMING {
        assert!(!codes.contains(&c), "option {c} sent: {codes:?}");
    }
}

#[test]
fn the_options_sent_are_the_ones_the_record_names() {
    // Message type, client id (the per-boot random MAC), maximum message size
    // and the request list; smoltcp's order. What the source comment says
    // differs from Windows, held here so a change to it is seen.
    let (codes, _) = first_discover();
    let mut sorted = codes.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![53, 55, 57, 61], "{codes:?}");
}
