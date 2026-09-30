// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

use crate::driver::Driver;
use crate::protocol::{parse, HDR_LEN, IPC_PAYLOAD_MAX};

use super::control;
use super::dispatch::dispatch;

const SERVICE_INBOX: u64 = 0;

pub fn run(mut driver: Driver) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        /*
         * The Wi-Fi control family the panels speak to any Wi-Fi driver comes
         * first; this driver's own protocol has a different tag.
         */
        if let Some(len) = control::answer(&rx[..n as usize], &mut tx) {
            let _ = mk_ipc_reply(sender_pid, tx.as_ptr(), len);
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut driver, sender_pid, req, body, &mut tx);
    }
}
