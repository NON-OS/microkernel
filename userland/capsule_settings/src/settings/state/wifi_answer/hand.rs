// NONOS Operating System (AGPL-3.0-or-later)

//! Handing a scan or join to a worker, or running it here.

use alloc::boxed::Box;

use super::super::state::State;
use super::super::wifi_pending::{Pending, Work};
use super::super::wifi_worker::{perform, send, Ask, Sent};
use super::apply::apply;

/// What the panel says when a request comes while the last is out.
const STILL_OUT: &str = "Still waiting for the driver's last answer; try again when it comes";

/// What the panel says after a request ran on the window thread.
const WAITED: &str = "No worker thread could start; the window waited for the driver";

/// Hand `ask` to the worker. The request stays on the panel ("Scanning...",
/// "Joining <name>...") until a tick finds the answer. With no worker to be
/// had, it runs here, as it did before workers, and the panel says so.
pub(in super::super) fn hand_off(state: &mut State, work: Work, ask: Ask) {
    match send(Box::new(ask)) {
        Sent::Out => state.wifi.pending = Pending::Out(work),
        Sent::Busy => state.wifi.notice = Some(STILL_OUT),
        Sent::Refused(ask) => {
            let answer = perform(&ask);
            drop(ask);
            apply(state, answer);
            if state.wifi.notice.is_none() {
                state.wifi.notice = Some(WAITED);
            }
        }
    }
}

/// While a request is out the driver is busy with it: refuse another, and
/// say why. True when refused.
pub fn refuse_while_out(state: &mut State) -> bool {
    if !state.wifi.pending.is_out() {
        return false;
    }
    state.wifi.notice = Some(STILL_OUT);
    true
}
