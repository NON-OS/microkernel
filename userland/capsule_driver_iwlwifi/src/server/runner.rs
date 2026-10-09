// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_wifi_core::netif::{wire, MAX_RESPONSE};

use crate::driver::Driver;
use crate::protocol::{parse, refused, HDR_LEN, IPC_PAYLOAD_MAX, E_BUSY, E_INVAL};

use super::control::{self, route, Heard, Route, View, OP_CONNECT, OP_DISCONNECT, REPLY_MAX};
use super::join_wire::{encode_code, encode_connect, encode_link};
use super::dispatch::dispatch;
use super::guard::drives_card;
use super::radio::Radio;
use super::respond;

const SERVICE_INBOX: u64 = 0;
/// How long the loop waits for a request while a scan runs before it pumps
/// the receive queue again (0 waits for the next request).
const SCAN_TICK_MS: u64 = 50;

pub fn run(mut driver: Driver) -> ! {
    /*
     * The gen3 radio comes up once, before the first request; on an adapter
     * this path does not run it records why and leaves the card to the
     * legacy protocol below.
     */
    let mut radio = Radio::start(&driver);
    // Large enough for net_core's largest frame as well as this driver's
    // own requests.
    let mut rx = vec![0u8; (HDR_LEN + IPC_PAYLOAD_MAX).max(wire::HDR_LEN + wire::MAX_FRAME)];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut reply = vec![0u8; REPLY_MAX];
    let mut link = vec![0u8; MAX_RESPONSE];
    loop {
        let mut sender_pid = 0u32;
        let wait = if radio.scanning() { SCAN_TICK_MS } else { 0 };
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender_pid);
        // A receive that failed at once sleeps in recv_ready, so the radio's
        // tick keeps its pace even if the inbox is gone.
        let ready = nonos_libc::recv_ready(n);
        radio.tick();
        if !ready || sender_pid == 0 {
            continue;
        }
        let n = n as usize;
        match route(&rx[..n], radio.can_join()) {
            Route::Link => {
                if let Some(len) = radio.serve_link(&rx[..n], &mut link) {
                    let _ = mk_ipc_reply(sender_pid, link.as_ptr(), len);
                }
                continue;
            }
            Route::Join(op) => {
                reply[..10].copy_from_slice(&rx[..10]);
                let len = match op {
                    OP_CONNECT => {
                        let r = radio.connect(&driver, &rx[10..n]);
                        // The body held the passphrase.
                        rx[..n].fill(0);
                        encode_connect(&mut reply, &r)
                    }
                    OP_DISCONNECT => encode_code(&mut reply, radio.disconnect()),
                    _ => encode_link(&mut reply, radio.link_info().as_ref()),
                };
                if let Some(len) = len {
                    let _ = mk_ipc_reply(sender_pid, reply.as_ptr(), len);
                }
                continue;
            }
            Route::Control | Route::Driver => {}
        }
        /*
         * The Wi-Fi control family the panels speak to any Wi-Fi driver comes
         * first; this driver's own protocol has a different tag.
         */
        if let Some(len) = control::answer(&rx[..n], &view(&radio), &mut reply) {
            let _ = mk_ipc_reply(sender_pid, reply.as_ptr(), len);
            continue;
        }
        // This driver's own requests are read as they were before the buffer
        // grew for net_core's frames: no longer than its own limit.
        let own = &rx[..n.min(HDR_LEN + IPC_PAYLOAD_MAX)];
        let Some((req, body)) = parse(own) else {
            let _ = respond::send(sender_pid, &refused(own), E_INVAL, &[], &mut tx);
            continue;
        };
        if radio.owns_card() && drives_card(req.op) {
            let _ = respond::send(sender_pid, &req, E_BUSY, &[], &mut tx);
            continue;
        }
        dispatch(&mut driver, sender_pid, req, body, &mut tx);
    }
}

fn view(radio: &Radio) -> View<'_> {
    let (stage, step, detail) = radio.stage();
    let (sweeps, stalls, frames) = radio.counts();
    let heard = radio.scanning().then(|| Heard {
        list: &radio.results,
        sweeps,
        stalls,
        frames,
        beacons: radio.beacons,
    });
    View { stage, step, detail, hw_rev: radio.hw_rev(), rf_id: radio.rf_id(), heard }
}
