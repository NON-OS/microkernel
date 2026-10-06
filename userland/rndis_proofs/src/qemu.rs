// NONOS Operating System (AGPL-3.0-or-later)
//! QEMU's usb-net in its RNDIS configuration as a scripted device: its
//! descriptors, and the control channel of usb_net_handle_control. A
//! command queues its completion; GET_ENCAPSULATED_RESPONSE takes the next
//! one, or one zero byte when none is queued. Each completion passes
//! through the test's `Tamper`, which may change, add to or withhold it.

use std::collections::VecDeque;

use nonos_usbnet::mock::{descriptors, Call, MockBus};
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC_STRING};

use crate::device::{init_cmplt, le, query_cmplt, set_cmplt, NIC_MAC};

/// (request type, QEMU's completion) to the messages queued instead.
pub type Tamper = Box<dyn FnMut(u32, Vec<u8>) -> Vec<Vec<u8>>>;

pub fn honest() -> Tamper {
    Box::new(|_, r| vec![r])
}

pub fn usb_net(mut tamper: Tamper) -> MockBus {
    let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
    MockBus::new(Box::new(move |call| {
        let configs: [&[u8]; 2] = [&CONFIG_RNDIS, &CONFIG_ECM];
        if let Some(bytes) = descriptors(call, &DEVICE, &configs, &[(3, &MAC_STRING)]) {
            return Ok(bytes);
        }
        match call {
            // QEMU takes encapsulated commands only on interface 0.
            Call::Out(s, m) if (s.request_type, s.request, s.index) == (0x21, 0x00, 0) => {
                if let Some(r) = complete(m)? {
                    queue.extend(tamper(le(m, 0), r));
                }
                Ok(vec![])
            }
            Call::In(s, len) if (s.request_type, s.request, s.index) == (0xA1, 0x01, 0) => {
                let r = queue.pop_front().unwrap_or(vec![0]);
                Ok(r[..r.len().min(*len)].to_vec())
            }
            _ => Ok(vec![]),
        }
    }))
}

/// rndis_parse: the completion of `m`, none for HALT, a STALL for a
/// message QEMU refuses or an information buffer past the message.
fn complete(m: &[u8]) -> Result<Option<Vec<u8>>, i32> {
    let id = le(m, 8);
    let fits = || {
        let (offs, len) = (le(m, 20) as usize + 8, le(m, 16) as usize);
        len <= m.len() && offs < m.len() && offs + len <= m.len()
    };
    match le(m, 0) {
        2 => Ok(Some(init_cmplt(id))),
        3 => Ok(None),
        4 if fits() => Ok(Some(match le(m, 12) {
            0x0101_0101 => query_cmplt(id, Some(&NIC_MAC)),
            0x0001_0114 => query_cmplt(id, Some(&[0; 4])),
            _ => query_cmplt(id, Some(&[])),
        })),
        5 if fits() => {
            Ok(Some(set_cmplt(id, if le(m, 12) == 0x0001_010E { 0 } else { 0xC000_00BB })))
        }
        _ => Err(-32),
    }
}
