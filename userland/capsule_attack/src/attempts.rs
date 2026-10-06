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

//! The attempts. Each is a raw syscall, so nothing in libc can stand between
//! the attempt and the kernel's answer.

use nonos_libc::mk_syscall_raw as raw;

use crate::line::{note, verdict};

use crate::codes::{MAST, MDBG, MISD, MISP, MMAP, MMMP, MPST, MSVL, MSVR, MUMP};

/// The kernel's half, mapped in every address space and never the user's.
const KERNEL: u64 = 0xFFFF_8000_0010_0000;
/// Lower half, never mapped here: where another capsule's heap sits in its own
/// address space, which is the only place that address means anything.
const ELSEWHERE: u64 = 0x0000_6000_0000_0000;
/// The broker's DMA buffer and MMIO register windows (broker/windows.rs).
const DMA_WINDOW: u64 = 0x0000_00A0_0000_0000;
const MMIO_WINDOW: u64 = 0x0000_0080_0000_0000;
/// This capsule's own service and reply endpoints (Capsule.mk).
const OWN_SERVICE: u64 = 4954;
const OWN_REPLY: u64 = 4955;

/// Without Network, open a socket through net.sockets; without Mmio or Admin,
/// ask for a device's MMIO window.
pub fn cap_escape() {
    // Every service that carries traffic takes Network
    // (src/services/registry/policy.rs), the anonymity ones included: a
    // capsule without it must not reach the network through any of them.
    for (name, what) in NETWORK_SERVICES {
        network_send(name, what);
    }
    let mut map = [0u8; 32];
    let rc = raw(MMMP, [0, 0, 0, 0, 4096, map.as_mut_ptr() as u64]);
    verdict(b"cap-escape", b"an MMIO window without Mmio, MkMmioMap", rc);
}

const NETWORK_SERVICES: [(&[u8], &[u8]); 6] = [
    (b"net.sockets", b"a socket through net.sockets without Network, MkIpcSend"),
    (b"net.tcp", b"a TCP connection through net.tcp without Network, MkIpcSend"),
    (b"net.dns", b"a name lookup through net.dns without Network, MkIpcSend"),
    (b"net.nym", b"a mixnet session through net.nym without Network, MkIpcSend"),
    (b"net.anon", b"an onion circuit through net.anon without Network, MkIpcSend"),
    (b"net.socks5", b"a SOCKS stream through net.socks5 without Network, MkIpcSend"),
];

fn network_send(name: &[u8], what: &[u8]) {
    if let Some((port, _)) = lookup(name) {
        let open = [0u8; 16];
        let rc = raw(MISD, [u64::from(port), open.as_ptr() as u64, open.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", what, rc);
    } else {
        note(b"[ATTACK-NOTE] cap-escape: a network service is not running, its send was not tried\n");
    }
}

/// A service's port and pid, when it is running.
fn lookup(name: &[u8]) -> Option<(u32, u32)> {
    let (mut port, mut pid) = (0u32, 0u32);
    let found = raw(MSVL, [name.as_ptr() as u64, name.len() as u64, &mut port as *mut u32 as u64, &mut pid as *mut u32 as u64, 0, 0]);
    (found == 0 && port != 0 && pid != 0).then_some((port, pid))
}

/// The registered names of the network cards' drivers
/// (`src/services/registry/held.rs`).
const CARDS: [&[u8]; 6] = [
    b"driver.virtio_net0",
    b"driver.e1000_0",
    b"driver.rtl8169_0",
    b"driver.rtl8139_0",
    b"driver.iwlwifi0",
    b"driver.rtl8821ce0",
];

/// Go round the gates by pid and straight to the card. Every endpoint a
/// process serves is read from its one inbox, so a send by pid into
/// net.sockets' inbox is a send to net.sockets; and a card's driver puts
/// what it is sent on the wire, past the stack and the chosen route, so only
/// the stack may reach it, by name or by pid.
pub fn inbox_escape() {
    let junk = [0u8; 16];
    match lookup(b"net.sockets") {
        Some((_, pid)) => {
            let rc = raw(MISP, [u64::from(pid), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
            verdict(b"cap-escape", b"net.sockets' inbox by its pid without Network, MkIpcSendToPid", rc);
        }
        None => note(b"[ATTACK-NOTE] cap-escape: net.sockets is not running, its inbox was not tried\n"),
    }
    let mut tried = false;
    for card in CARDS {
        let Some((port, pid)) = lookup(card) else { continue };
        tried = true;
        let rc = raw(MISD, [u64::from(port), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", b"a network card's driver by name, held to the stack, MkIpcSend", rc);
        let rc = raw(MISP, [u64::from(pid), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", b"a network card's driver by its pid, held to the stack, MkIpcSendToPid", rc);
    }
    if !tried {
        note(b"[ATTACK-NOTE] cap-escape: no network card's driver is running, its sends were not tried\n");
    }
    let mut tried = false;
    for device in DEVICES {
        let Some((port, pid)) = lookup(device) else { continue };
        tried = true;
        let rc = raw(MISD, [u64::from(port), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", b"a device's driver by name, held to its service, MkIpcSend", rc);
        let rc = raw(MISP, [u64::from(pid), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", b"a device's driver by its pid, held to its service, MkIpcSendToPid", rc);
    }
    if !tried {
        note(b"[ATTACK-NOTE] cap-escape: no held device driver is running, its sends were not tried\n");
    }
}

/// The other drivers held to their services: keyboards and pointers (read
/// or typed into), the random source, USB storage, the USB and I2C
/// controllers (raw transfers), the GPU (the whole screen) and the sound
/// card.
const DEVICES: [&[u8]; 9] = [
    b"driver.ps2_kbd0",
    b"driver.usb_hid0",
    b"driver.i2c_hid0",
    b"driver.usb_msc0",
    b"driver.virtio_rng",
    b"driver.xhci0",
    b"driver.i2c_pci0",
    b"driver.virtio_gpu0",
    b"driver.hda0",
];

/// Hand the kernel addresses this capsule does not own.
pub fn foreign_memory() {
    let rc = raw(MDBG, [KERNEL, 16, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"a kernel address to MkDebug", rc);
    let rc = raw(MDBG, [ELSEWHERE, 16, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"another address space's address to MkDebug", rc);
    let rc = raw(MISD, [OWN_REPLY, KERNEL, 16, 0, 0, 0]);
    verdict(b"foreign-memory", b"a kernel address as an IPC message, MkIpcSend", rc);
    let rc = raw(MMAP, [KERNEL, 4096, 0x3, 0, 0, 0]);
    verdict(b"foreign-memory", b"a fixed mapping at a kernel address, MkMmap", rc);
    let rc = raw(MDBG, [0x1000, u64::MAX, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"a length that wraps the address space, MkDebug", rc);
    let rc = raw(MPST, [KERNEL, 4, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"the process table written to a kernel address, MkProcStat", rc);
    let rc = raw(MAST, [KERNEL, 0, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"the boot record written to a kernel address, MkAttestStatus", rc);
    let rc = raw(MUMP, [KERNEL, 4096, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"a kernel page unmapped, MkMunmap", rc);
    // The broker's windows are given back only through the broker; a frame
    // freed by munmap there would stay reachable by the device that has it.
    let rc = raw(MUMP, [DMA_WINDOW, 4096, 0, 0, 0, 0]);
    verdict(b"foreign-memory", b"a page of the DMA buffer window unmapped, MkMunmap", rc);
    let rc = raw(MMAP, [MMIO_WINDOW, 4096, 0x3, 0, 0, 0]);
    verdict(b"foreign-memory", b"a fixed mapping in the MMIO register window, MkMmap", rc);
}

/// Register a service without RegisterService: net.dns, a name the kernel
/// lets a holder of the right claim at run time, so the missing right is the
/// only reason left to refuse it (`ipc/register_allowed.rs`).
pub fn register_service() {
    let name = b"net.dns";
    let rc = raw(MSVR, [name.as_ptr() as u64, name.len() as u64, OWN_SERVICE, 0, 0, 0]);
    verdict(b"cap-escape", b"register net.dns without RegisterService, MkServiceRegister", rc);
}

/// Claim names other capsules' manifests own.
pub fn service_squat() {
    for name in [&b"net.sockets"[..], &b"app.file_manager"[..]] {
        let rc = raw(MSVR, [name.as_ptr() as u64, name.len() as u64, OWN_SERVICE, 0, 0, 0]);
        let what: &[u8] = if name.len() == 11 { b"MkServiceRegister net.sockets" } else { b"MkServiceRegister app.file_manager" };
        verdict(b"service-squat", what, rc);
    }
}
