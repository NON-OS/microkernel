use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

/* on-screen bring-up sentinels, silenced
const KIND_DRIVER_READY: u16 = 0xFE01;
const KIND_TOUCHPAD_FOUND: u16 = 0xFE02;
const KIND_CONTROLLER_BOUND: u16 = 0xFE04;
const KIND_BIND_CONFIRMED: u16 = 0xFE05;
const KIND_WAKE_CONFIRMED: u16 = 0xFE07;

fn signal(kind: u16) {
    let ev =
        InputEvent { kind, flags: 0, code: 0, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    let _ = mk_input_event_post(&ev);
}
*/

use super::dispatch::dispatch;
use crate::input;
use crate::protocol::{parse, refused, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::respond;
use crate::state::State;

const SERVICE_INBOX: u64 = 0;
const RECV_TIMEOUT_MS: u64 = 2;
// A touchpad may not answer the very first probe at startup: the I2C
// controller has just come up, and the device needs a moment to wake. So
// keep re-probing on a slow cadence until it is found, instead of giving up
// after the single startup attempt and never producing input.
const REPROBE_EVERY: u32 = 250;
// Cycles (~2ms each) of doorbell silence after which its trust is revoked
// and timed polling resumes; a genuine idle pad re-proves itself on the very
// next touch, so the only cost is one re-verification read.
const DOORBELL_TRUST_CYCLES: u32 = 2500;
const DOORBELL_GIVE_UP: u32 = 3;
const DOORBELL_MISS_LIMIT: u32 = 16;

pub fn run(mut state: State) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut ticks: u32 = 0;
    let mut quiet_cycles: u32 = 0;
    /* bring-up sentinels, silenced
    let mut found_signaled = false;
    signal(KIND_DRIVER_READY);
    if let Some((slot, confirmed)) = crate::i2c_client::query_driver_info(state.i2c_port) {
        for _ in 0..slot {
            signal(KIND_CONTROLLER_BOUND);
        }
        if confirmed {
            signal(KIND_BIND_CONFIRMED);
        }
    }
    */
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_TIMEOUT_MS,
            &mut sender_pid,
        );
        // A receive that failed without waiting sleeps here, so the polls
        // below keep their pace even if the inbox is gone.
        let ready = nonos_libc::recv_ready(n);
        if !state.found() {
            ticks = ticks.wrapping_add(1);
            if ticks.is_multiple_of(REPROBE_EVERY) {
                crate::setup::reprobe(&mut state);
            }
        }
        /* bring-up sentinels, silenced
        if state.found() && !found_signaled {
            signal(KIND_TOUCHPAD_FOUND);
            if state.woke {
                signal(KIND_WAKE_CONFIRMED);
            }
            found_signaled = true;
        }
        */
        // Interrupt-paced reads when the platform gives us the doorbell: the
        // pad holds its interrupt line active while a fresh report waits, so
        // a quiet doorbell means no i2c read at all: no stale re-reads, no
        // torn frames. The doorbell must fire once before it is trusted
        // (timed polling continues until then, so a line the firmware never
        // wired cannot silence input), and trust decays after a long quiet
        // stretch so a one-off spurious reading cannot lock the gate shut.
        let mut do_poll = true;
        let mut rang = false;
        if state.found() && !state.doorbell_absent {
            match crate::i2c_client::query_doorbell(state.i2c_port) {
                Some((present, fired)) => {
                    state.doorbell_failures = 0;
                    if !present {
                        // The platform's GPIO layout is not mapped: reads
                        // go by timer from now on, without asking again.
                        state.doorbell_absent = true;
                    } else if fired {
                        state.doorbell_proven = true;
                        quiet_cycles = 0;
                        rang = true;
                    } else if state.doorbell_proven {
                        quiet_cycles = quiet_cycles.saturating_add(1);
                        if quiet_cycles > DOORBELL_TRUST_CYCLES {
                            state.doorbell_proven = false;
                        } else {
                            do_poll = false;
                        }
                    }
                }
                None => {
                    // A controller that refuses the op (or keeps timing out)
                    // has no doorbell to offer.
                    state.doorbell_failures = state.doorbell_failures.saturating_add(1);
                    if state.doorbell_failures >= DOORBELL_GIVE_UP {
                        state.doorbell_absent = true;
                    }
                }
            }
        }
        if do_poll {
            input::poll(&mut state);
            if rang {
                if state.last_read_had_report {
                    state.doorbell_misses = 0;
                } else {
                    state.doorbell_misses = state.doorbell_misses.saturating_add(1);
                    if state.doorbell_misses >= DOORBELL_MISS_LIMIT {
                        state.doorbell_absent = true;
                        state.doorbell_proven = false;
                        crate::diag::line(alloc::format!(
                            "[i2chid] doorbell rang {} times with no report; reads go back to the timer\n",
                            DOORBELL_MISS_LIMIT
                        ));
                    }
                }
            }
        }
        if !ready || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else {
            let _ = respond::send(sender_pid, &refused(&rx[..n as usize]), E_INVAL, &[], &mut tx);
            continue;
        };
        dispatch(&mut state, sender_pid, req, body, &mut tx);
    }
}
