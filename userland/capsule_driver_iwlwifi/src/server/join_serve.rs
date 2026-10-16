// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! A join op the radio runs: connect, disconnect, or the link's state.

use crate::driver::Driver;
use nonos_wifi_core::wipe::wipe;

use super::control::{OP_CONNECT, OP_DISCONNECT};
use super::join_wire::{encode_code, encode_connect, encode_link_with};
use super::radio::Radio;

/// The reply's length in `reply`; `req` is the whole request, wiped once a
/// connect has read the passphrase in it.
pub(super) fn join(
    radio: &mut Radio,
    driver: &Driver,
    op: u16,
    req: &mut [u8],
    reply: &mut [u8],
) -> Option<usize> {
    reply[..10].copy_from_slice(&req[..10]);
    match op {
        OP_CONNECT => {
            let r = radio.connect(driver, &req[10..]);
            wipe(req);
            radio.note_join(r.code);
            encode_connect(reply, &r)
        }
        OP_DISCONNECT => {
            radio.note_left();
            encode_code(reply, radio.disconnect())
        }
        _ => encode_link_with(
            reply,
            radio.link_info().as_ref(),
            &radio.history(),
            radio.transition_disable(),
        ),
    }
}
