// NONOS Operating System (AGPL-3.0-or-later)
//! nonos_usb_hub's walk run over this host's wire, as a class driver runs
//! it: every control request, hub op and slot goes through the driver's
//! own dispatch to the modelled controller, and time passes as frames.
//! QEMU's usb-hub is the lane this stands in for.

use nonos_usb_hub::{Host, Place, Setup, StatusEndpoint};

use crate::env::Clock;
use crate::model::{call, Cx};
use crate::wire::ops::*;

pub struct Lane<'a> {
    pub cx: &'a mut Cx,
    pub lines: Vec<String>,
}

fn done(status: i32) -> Result<(), i32> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

impl Host for Lane<'_> {
    fn control(&mut self, slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
        let [v0, v1] = s.value.to_le_bytes();
        let [i0, i1] = s.index.to_le_bytes();
        let [l0, l1] = s.length.to_le_bytes();
        let mut body = vec![slot, 0, s.request_type, s.request, v0, v1, i0, i1, l0, l1];
        if s.request_type & 0x80 == 0 {
            body.extend_from_slice(&data[..s.length as usize]);
        }
        let r = call(self.cx, OP_CONTROL_TRANSFER, &body);
        done(r.status)?;
        let n = u16::from_le_bytes([r.data[0], r.data[1]]) as usize;
        if s.request_type & 0x80 != 0 {
            data[..n].copy_from_slice(&r.data[2..2 + n]);
        }
        Ok(n)
    }
    fn configure_hub(&mut self, slot: u8, ports: u8, think: u8) -> Result<(), i32> {
        done(call(self.cx, OP_CONFIGURE_HUB, &[slot, ports, think, 0]).status)
    }
    fn status_endpoint(&mut self, slot: u8, ep: StatusEndpoint) -> Result<(), i32> {
        let [m0, m1] = ep.max_packet.to_le_bytes();
        let body = [slot, ep.address, m0, m1, ep.interval, 0];
        done(call(self.cx, OP_ALLOC_TRANSFER_RING, &body).status)
    }
    fn enable_slot(&mut self) -> Result<u8, i32> {
        let r = call(self.cx, OP_ENABLE_SLOT, &[]);
        done(r.status).map(|_| r.data[0])
    }
    fn address_routed(&mut self, slot: u8, p: &Place) -> Result<(), i32> {
        let [r0, r1, r2, r3] = p.route.to_le_bytes();
        let (ts, tp) = p.tt.map_or((0, 0), |t| (t.hub_slot, t.port));
        let body = [slot, p.root_port, p.speed, r0, r1, r2, r3, ts, tp];
        done(call(self.cx, OP_ADDRESS_ROUTED, &body).status)
    }
    fn disable_slot(&mut self, slot: u8) {
        let _ = call(self.cx, OP_DISABLE_SLOT, &[slot]);
    }
    fn idle_ms(&mut self, ms: u64) {
        (0..ms).for_each(|_| self.cx.clock.idle());
    }
    fn now_ms(&mut self) -> i64 {
        self.cx.clock.now_ms() as i64
    }
    fn say(&mut self, line: &[u8]) {
        self.lines.push(String::from_utf8_lossy(line).into_owned());
    }
}
