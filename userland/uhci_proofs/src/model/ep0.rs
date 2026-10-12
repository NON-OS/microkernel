// NONOS Operating System (AGPL-3.0-or-later)
//! EP0: a SETUP starts a request and sets both directions' next toggle to
//! DATA1; data then flows in the request's direction, and the status stage
//! is the one packet the other way. SET_ADDRESS takes effect after its
//! status stage. A request named in `stall_request` stalls past SETUP.

use super::device::{Device, Reply};
use super::pid::{PID_IN, PID_OUT, PID_SETUP};

#[derive(Default)]
pub struct Ep0 {
    setup: Option<[u8; 8]>,
    data: Vec<u8>,
    new_addr: Option<u8>,
    stalled: bool,
}

pub fn packet(d: &mut Device, pid: u8, toggle: bool, out: &[u8], max: usize) -> Reply {
    let input = d.ep0.setup.map(|s| s[0] & 0x80 != 0);
    match (pid, input) {
        (PID_SETUP, _) if out.len() == 8 => {
            if toggle {
                d.toggle_errors += 1;
            }
            let s: [u8; 8] = out.try_into().unwrap();
            d.requests.push(s);
            d.ep0 =
                Ep0 { setup: Some(s), stalled: d.stall_request == Some(s[1]), ..Ep0::default() };
            d.toggles[0..2].fill(true);
            prepare(d, s);
            Reply::Ack(vec![])
        }
        (_, None) => Reply::Stall,
        _ if d.ep0.stalled => Reply::Stall,
        (PID_IN, Some(true)) => {
            let n = d.ep0.data.len().min(d.mps0 as usize).min(max);
            let chunk = d.ep0.data.drain(..n).collect();
            d.sent(1, toggle, chunk)
        }
        (PID_OUT, Some(false)) => {
            if d.expect(0, toggle) {
                d.ctrl_out.extend_from_slice(out);
            }
            Reply::Ack(vec![])
        }
        // The status stage: a zero-length packet, DATA1.
        (PID_IN | PID_OUT, Some(_)) => {
            if !toggle || !out.is_empty() {
                d.toggle_errors += 1;
            }
            if let Some(a) = d.ep0.new_addr.take() {
                d.addr = a;
            }
            d.ep0.setup = None;
            Reply::Ack(vec![])
        }
        _ => Reply::Stall,
    }
}

fn prepare(d: &mut Device, s: [u8; 8]) {
    let len = u16::from_le_bytes([s[6], s[7]]) as usize;
    let data: Vec<u8> = match (s[0], s[1], s[3]) {
        (0x80, 6, 1) => d.desc.to_vec(),
        (0x80, 6, 2) => d.config.clone(),
        (0x00, 5, _) => {
            d.ep0.new_addr = Some(s[2]);
            vec![]
        }
        // A vendor IN request: its bytes count up from zero.
        (0xC0, _, _) => (0..len).map(|i| i as u8).collect(),
        _ => vec![],
    };
    d.ep0.data = data.into_iter().take(len).collect();
}
