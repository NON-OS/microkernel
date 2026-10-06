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

/*
 * Opening the shield for this account. If the service already holds this
 * account's store open, the wallet only reads it again. Otherwise the
 * keyring gives the recovery words, which go straight to the service and
 * are zeroed here; an account imported from a key has no words, and its
 * key goes the same way. The service then checks the account it derived
 * is this one, and refuses to open anything else.
 */

use alloc::string::String;

use shield_wire::*;

use nonos_libc::mk_uptime_ms;

use super::client::{call, NO_ANSWER};
use crate::wallet::ipc::{export_secret, shield_words};
use crate::wallet::screen::receive_address::address_hex;
use crate::wallet::state::{State, WORDS_NONE, WORDS_UNREAD};

const NO_ACCOUNT: &str = "Make or restore an account first. The shield opens from its words.";
const NO_WORDS: &str = "The keyring would not hand this account's words to the shield.";
const NO_KEYRING: i32 = -2;
/* The keyring took longer than the call allows, as while it unseals. */
const KEYRING_SLOW: i32 = -11;
const KEYRING_LATE: &str = "The keyring did not answer in time. The shield is asked to open \
     again by itself in a few seconds.";
const WORDS_UNREAD_YET: &str = "This account's recovery words have not been read from this \
     machine yet. The shield opens from them once they are, and is asked again by itself.";
/* Said by every shield step asked off the one network the pool runs on. */
pub const ELSEWHERE: &str = "The shield pool is not deployed on Ethereum mainnet. It runs on \
     Sepolia: switch there to use it.";

/* An open is a few file writes and under a second of key derivation, even
 * on an emulated machine. Past this it is stuck, not slow. */
pub const OPEN_STUCK_S: u32 = 60;

/* What an open's wait says after its elapsed time: where the address will
 * appear, or, past `OPEN_STUCK_S`, that the service is stuck at the step
 * named. Any other job says nothing more. */
pub fn opening_said(op: u16, elapsed_s: u32) -> &'static str {
    if !matches!(op, OP_OPEN_WORDS | OP_OPEN_KEY) {
        return "";
    }
    if elapsed_s < OPEN_STUCK_S {
        " The private address appears here when it is done, in a few seconds."
    } else {
        " This takes seconds, so the shield service is stuck at that step, not slow."
    }
}

/* Whether the picked network has the shield pool. */
pub fn here() -> bool {
    crate::wallet::chain::current().shield
}

/* Whether the service already holds this account open; reads it if so. */
fn held(
    state: &mut State,
    account: &str,
    running: &mut Option<String>,
) -> Result<bool, &'static str> {
    let a = call(OP_STATE, &[])?;
    let open = field(&a.body, "unlocked") == Some("1")
        && field(&a.body, "account").is_some_and(|x| x.eq_ignore_ascii_case(account));
    if open {
        super::apply::state(state, &a.body);
        *running = field(&a.body, "job").map(String::from);
    }
    Ok(open)
}

/* The longest wait between two asks of a service that does not answer. */
const RETRY_MAX_MS: i64 = 30_000;

/* No answer in time: ask again later, by itself, waiting longer each time. */
fn later(state: &mut State) {
    let ui = &mut state.shield_ui;
    let wait = (2_000i64 << ui.tries.min(4)).min(RETRY_MAX_MS);
    ui.tries = ui.tries.saturating_add(1);
    ui.retry_at_ms = Some(mk_uptime_ms() + wait);
    ui.failure = None;
}

/* The service lost its job and its open store, as when it restarted. */
pub fn restarted(state: &mut State) {
    later(state);
}

/* The service answered: no ask is pending. */
fn answered(state: &mut State) {
    state.shield_ui.retry_at_ms = None;
    state.shield_ui.tries = 0;
}

/* The pending ask, once its time has come. True when the screen should repaint. */
pub fn retry_due(state: &mut State) -> bool {
    match state.shield_ui.retry_at_ms {
        Some(at) if mk_uptime_ms() >= at && state.shield_ui.waiting.is_none() => {
            state.shield_ui.retry_at_ms = None;
            if here() && state.address_ready {
                probe_then_open(state);
            }
            true
        }
        _ => false,
    }
}

/* Whether the wallet is waiting to ask a slow service again. */
pub fn retrying(state: &State) -> bool {
    state.shield_ui.retry_at_ms.is_some()
}

pub fn open(state: &mut State) {
    if state.shield_ui.waiting.is_some() {
        return;
    }
    if !here() {
        state.shield_ui.failure = Some(String::from(ELSEWHERE));
        return;
    }
    if !state.address_ready {
        state.shield_ui.failure = Some(String::from(NO_ACCOUNT));
        return;
    }
    let account = address_hex(state);
    /* Asked before the words are: a service that does not answer never gets them. */
    let mut running = None;
    match held(state, &account, &mut running) {
        Err(NO_ANSWER) => return later(state),
        Err(why) => {
            state.shield_ui.failure = Some(String::from(why));
            return;
        }
        Ok(true) => {
            answered(state);
            state.shield_ui.opened = true;
            /* A job still running for this wallet, as after the window
             * restarted mid-proof, is waited on, not started over. The pool
             * is read again only when the last read is a sync interval old,
             * so going in and out of Shield never holds every button grey. */
            let resumed = running.as_deref().is_some_and(|name| super::job::resume(state, name));
            if !resumed && super::follow::sync_stale(&state.shield_ui, mk_uptime_ms()) {
                state.shield_ui.last_sync_ms = mk_uptime_ms();
                super::job::start(state, OP_SYNC, &[]);
            }
            return;
        }
        Ok(false) => answered(state),
    }
    state.shield_ui.opened = false;
    /* Words not read yet are read again first; the key never stands in. */
    if state.words == WORDS_UNREAD {
        crate::wallet::accounts::words(state);
    }
    let (port, owner, wallet) = (state.keyring_port, state.owner_pid, state.wallet_id);
    match shield_words(port, owner, wallet) {
        Ok(words) => {
            if !super::job::start(state, OP_OPEN_WORDS, &[&account, words.text()]) {
                slow(state);
            }
        }
        /* Only a wallet from a key opens from it: its nox1 is the key's.
         * A wallet with words opened from its key would get an address
         * its words could not restore. */
        Err(NO_KEYRING) if state.words == WORDS_NONE => open_key(state, &account),
        Err(NO_KEYRING) if state.words == WORDS_UNREAD => {
            later(state);
            state.shield_ui.failure = Some(String::from(WORDS_UNREAD_YET));
        }
        Err(KEYRING_SLOW) => keyring_late(state),
        Err(_) => state.shield_ui.failure = Some(String::from(NO_WORDS)),
    }
}

fn open_key(state: &mut State, account: &str) {
    let mut key = match export_secret(state.keyring_port, state.owner_pid, state.wallet_id) {
        Ok(key) => key,
        Err(KEYRING_SLOW) => return keyring_late(state),
        Err(_) => {
            state.shield_ui.failure = Some(String::from(NO_WORDS));
            return;
        }
    };
    let mut text = String::with_capacity(66);
    text.push_str("0x");
    for b in key.iter() {
        text.push(char::from(HEX[(b >> 4) as usize]));
        text.push(char::from(HEX[(b & 15) as usize]));
    }
    if !super::job::start(state, OP_OPEN_KEY, &[account, &text]) {
        slow(state);
    }
    // SAFETY: zero bytes are valid UTF-8, and the string is dropped right after.
    for b in unsafe { text.as_bytes_mut() } {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    for b in key.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
}

const HEX: &[u8; 16] = b"0123456789abcdef";

/* A keyring too slow to answer is asked again later, not taken as a refusal. */
fn keyring_late(state: &mut State) {
    later(state);
    state.shield_ui.failure = Some(String::from(KEYRING_LATE));
}

/* An open that went unanswered is asked again; any other refusal stands. */
fn slow(state: &mut State) {
    if state.shield_ui.failure.as_deref() == Some(NO_ANSWER) {
        later(state);
    }
}

/* Open the shield for this account if it is not open or opening, so its
 * private address is there wherever it is shown. Quiet when no service
 * runs: the screen asking says so itself. */
pub fn ensure(state: &mut State) {
    if !here() || state.shield_ui.opened || state.shield_ui.waiting.is_some() || retrying(state) {
        return;
    }
    probe_then_open(state);
}

/* Look for the service and open the shield if it is there. A service not
 * up yet, as early in a boot, is looked for again by itself, waiting longer
 * each time (`later`), so the shield never stays closed for want of one look. */
fn probe_then_open(state: &mut State) {
    state.shield = super::probe::probe();
    if state.shield.available() {
        open(state);
    } else {
        later(state);
    }
}

/* The wallet locked: the service forgets the store's keys too, and stops a
 * proof under way. It answers at once, whatever the store is doing. */
pub fn lock(state: &mut State) {
    let _ = call(OP_LOCK, &[]);
    let ui = &mut state.shield_ui;
    ui.waiting = None;
    ui.misses = 0;
    ui.poll_at_ms = 0;
    ui.phase = None;
    ui.permille = None;
    ui.opened = false;
    ui.retry_at_ms = None;
    ui.tries = 0;
    ui.nox1 = None;
    ui.held = [None, None];
    ui.review = None;
    ui.quote = None;
    ui.live = false;
    ui.wait_left = None;
    ui.withdraw_to = None;
}

/* Another account is open: nothing of the last one's shield is kept, its
 * history and the spend it followed included. */
pub fn forget_account(state: &mut State) {
    lock(state);
    state.shield_ui = crate::wallet::state::shield_ui::ShieldUi::default();
}

/* The network changed: the store is closed and nothing it said is shown
 * on the other network. The history and the spend being followed stay,
 * since only the shield's network has them, and are read again once the
 * store opens there. */
pub fn forget_network(state: &mut State) {
    lock(state);
    let ui = &mut state.shield_ui;
    ui.screen = crate::wallet::state::shield_ui::SHIELD_HOME;
    ui.phase = None;
    ui.permille = None;
    ui.news = None;
    ui.failure = None;
}
