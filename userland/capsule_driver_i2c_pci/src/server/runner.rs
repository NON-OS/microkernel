use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::driver::Driver;
use crate::protocol::{
    parse, refused, wipe, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_ACPI_HID,
    OP_CONTROLLER_INFO, OP_GPIO_DOORBELL, OP_HEALTHCHECK, OP_PROBE, OP_REGISTER_SNAPSHOT,
    OP_TIMING_INFO, OP_TRANSFER,
};
use crate::server::{handlers, respond};

const SERVICE_INBOX: u64 = 0;

pub fn run(driver: Driver) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if !nonos_libc::recv_ready(n) {
            continue;
        }
        // Clamped, so the wipe below covers the request whatever n says.
        let got = (n as usize).min(rx.len());
        let msg = &mut rx[..got];
        if sender_pid != 0 {
            answer(&driver, sender_pid, msg, &mut tx);
        }
        // The request goes once it is handled, a transfer's write bytes
        // with it; respond::send wipes each reply.
        wipe(msg);
    }
}

fn answer(driver: &Driver, sender_pid: u32, msg: &[u8], tx: &mut [u8]) {
    let Some((req, body)) = parse(msg) else {
        let _ = respond::send(sender_pid, &refused(msg), E_INVAL, &[], tx);
        return;
    };
    dispatch(driver, sender_pid, req, body, tx);
}

fn dispatch(
    driver: &Driver,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_CONTROLLER_INFO if body.is_empty() => {
            handlers::controller::handle(driver, sender_pid, &req, tx)
        }
        OP_REGISTER_SNAPSHOT if body.is_empty() => {
            handlers::snapshot::handle(driver, sender_pid, &req, tx)
        }
        OP_TIMING_INFO if body.is_empty() => handlers::timing::handle(driver, sender_pid, &req, tx),
        OP_TRANSFER => handlers::transfer::handle(driver, sender_pid, &req, body, tx),
        OP_PROBE => handlers::probe::handle(driver, sender_pid, &req, body, tx),
        OP_ACPI_HID if body.is_empty() => handlers::acpi_hid::handle(driver, sender_pid, &req, tx),
        OP_GPIO_DOORBELL if body.is_empty() => {
            handlers::doorbell::handle(driver, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
