/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! `driver.mt7921e0`: the MediaTek MT7921 and MT7922 PCIe Wi-Fi capsule.
//! Its firmware is not in it: the files come from the firmware package on
//! the boot medium, through the kernel, checked against the hashes it
//! carries (`fw::pins`).

#![no_std]
#![no_main]

extern crate alloc;

mod bringup;
mod card;
mod clock;
mod dma;
mod fw;
mod hw;
mod ids;
mod init;
mod mac;
mod mcu;
mod power;
mod radio;
mod say;
mod serve;
mod setup;
mod stage;
mod vif;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.mt7921e";

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh
/// stack with the capsule's heap reserved; it must never be called from
/// Rust.
///
/// Without a supported card the driver says so and leaves (`EXIT_ABSENT`)
/// before claiming anything; a card it cannot claim is retried on the
/// shared schedule. A claimed card is served whatever its bring-up
/// reached, so the panel can say where it stopped.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let mapped = match start_driver(DRIVER, setup::find(), |f| setup::claim(*f)) {
        Ok(m) => m,
        Err(code) => mk_exit(code),
    };
    let (radio, stage) = bringup::run(&mapped);
    serve::run(radio, stage);
}
