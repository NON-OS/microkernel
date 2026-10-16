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

use super::control::{self, route, Route, REPLY_MAX};
use super::join_serve::join;
use super::link_serve::link_serve;
use super::radio::Radio;
use super::view::view;

const SERVICE_INBOX: u64 = 0;
/// How long the loop waits for a request while a scan runs or a link is up
/// before it services them again; otherwise it waits for the next request.
const SCAN_TICK_MS: u64 = 50;

pub fn run(mut driver: Driver) -> ! {
    // The gen3 radio comes up once, before the first request; on an adapter
    // this path does not run it records why, for the status reply.
    let mut radio = Radio::start(&mut driver);
    // Large enough for net_core's largest frame; a control request (a join
    // body is at most 99 bytes) is far shorter.
    let mut rx = vec![0u8; wire::HDR_LEN + wire::MAX_FRAME];
    let mut reply = vec![0u8; REPLY_MAX];
    let mut link = vec![0u8; MAX_RESPONSE];
    loop {
        let mut sender_pid = 0u32;
        let wait = if radio.scanning() || radio.joined() { SCAN_TICK_MS } else { 0 };
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
                if let Some(len) = link_serve(&mut radio, sender_pid, &rx[..n], &mut link) {
                    let _ = mk_ipc_reply(sender_pid, link.as_ptr(), len);
                }
                continue;
            }
            Route::Join(op) => {
                if let Some(len) = join(&mut radio, &driver, op, &mut rx[..n], &mut reply) {
                    let _ = mk_ipc_reply(sender_pid, reply.as_ptr(), len);
                }
                continue;
            }
            Route::Control => {
                if let Some(len) = control::answer(&rx[..n], &view(&radio), &mut reply) {
                    let _ = mk_ipc_reply(sender_pid, reply.as_ptr(), len);
                }
            }
            // Nothing in the tree speaks anything else to this driver.
            Route::Other => {}
        }
    }
}
