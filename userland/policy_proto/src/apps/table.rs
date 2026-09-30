/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* The apps setup lists, and the dock services each switch covers. */

use super::bits::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, STORE, WALLET};

pub struct App {
    pub bit: u8,
    pub name: &'static [u8],
    /* One line: what the app is for. */
    pub purpose: &'static [u8],
    /* The dock services it covers; none for an app without a dock icon. */
    pub services: &'static [&'static [u8]],
}

const fn app(
    bit: u8,
    name: &'static [u8],
    purpose: &'static [u8],
    s: &'static [&'static [u8]],
) -> App {
    App { bit, name, purpose, services: s }
}

/* In the order setup lists them. Each is on unless the person turns it off. */
pub const OPTIONAL: [App; 8] = [
    app(BROWSER, b"Browser", b"Web pages over the network", &[b"app.browser"]),
    app(WALLET, b"Wallet", b"NONOS wallet: keys and payments", &[b"app.nonos_wallet"]),
    app(STORE, b"App store", b"Browse and install signed apps", &[b"app.store"]),
    app(FILES, b"Files", b"Browse and open files", &[b"app.file_manager"]),
    app(EDITOR, b"Text editor", b"Write and edit text files", &[b"app.text_editor"]),
    app(CALCULATOR, b"Calculator", b"Arithmetic", &[b"app.calculator"]),
    app(MEDIA, b"Media", b"Music, video and image viewers", MEDIA_SERVICES),
    app(LINUX, b"Linux and Qwen", b"Linux programs and the Qwen model", &[]),
];

const MEDIA_SERVICES: &[&[u8]] = &[b"app.audio_player", b"app.video_player", b"app.image_viewer"];

/* What the system cannot run without, shown by setup as always on. */
pub const REQUIRED: [&[u8]; 3] = [
    b"The desktop, Terminal, Settings and Processes",
    b"The file store, keys and crypto services",
    b"Drivers for the hardware found at boot",
];

/* The switch covering `service`, or 0 for a service no switch covers. */
pub fn bit_for_service(service: &[u8]) -> u8 {
    OPTIONAL.iter().filter(|a| a.services.contains(&service)).fold(0, |bits, a| bits | a.bit)
}

/* Whether `service` is turned off in `off`. */
pub fn is_off(off: u8, service: &[u8]) -> bool {
    off & bit_for_service(service) != 0
}
