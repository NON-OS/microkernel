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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::net::read_snapshot::Snapshot;
use crate::wallet::net::step::{Finished, Job};
use crate::wallet::state::State;

/// Refresh the account, a step per tick. The expensive network
/// self-diagnostic (DNS, sockets, the full TLS check) runs once to confirm
/// the link; after that every refresh is a single batched round trip that
/// fetches balance, nonce, fee and the staking figures over one connection.
/// The payments still on their way, on this network and account, are read
/// first, all in one batch with the newest block and the account's nonces. No step waits on
/// the network longer than a slice (`net::step`), and only a finished job
/// touches the state. Returns Repaint only when a shown value actually
/// changed, so a steady screen does not recomposite every cycle. On a
/// failed fetch the link is marked degraded so the next refresh re-runs the
/// diagnostic and reconnects. `due` begins a refresh when none is under way.
pub fn probe_tick(state: &mut State, due: bool) -> EventOutcome {
    if state.net_job.is_none() {
        if !due {
            return EventOutcome::Idle;
        }
        state.probe_step = 0;
        let chain = crate::wallet::chain::current().id;
        let open = state.sent.open(chain, &state.address);
        state.net_job = Some(if open.is_empty() {
            account_job(state)
        } else {
            Job::follow(open, &state.address)
        });
    }
    let Some(finished) = state.net_job.as_mut().and_then(Job::step) else {
        return EventOutcome::Idle;
    };
    state.net_job = None;
    outcome(finish(state, finished))
}

/// The diagnostic while the link is unconfirmed, the snapshot after.
fn account_job(state: &State) -> Job {
    /* A host is read from only once it answered this network's chain id. */
    if state.net.rpc_chain_ok && state.net.rpc_host == crate::wallet::chain::rpc_host() {
        Job::snapshot(&state.address)
    } else {
        Job::probe()
    }
}

/// Take a finished job's result, returning whether anything shown changed.
fn finish(state: &mut State, finished: Finished) -> bool {
    match finished {
        Finished::Probe(net) => {
            let changed = net.status != state.status;
            state.net = net;
            state.status = state.net.status;
            changed
        }
        Finished::Snapshot { snap, address, chain, host } => {
            /* Asked for an account or a network no longer open. */
            if address != state.address || chain != crate::wallet::chain::current().id {
                return false;
            }
            match snap {
                Some(snap) => {
                    /* A new block read is news on Home, whatever else moved. */
                    let mut newer = false;
                    if let Some(block) = snap.head {
                        let route = state.net.route.map_or("", |r| r.label());
                        let at_ms = nonos_libc::mk_uptime_ms();
                        let read =
                            crate::wallet::net::last_read::LastRead { block, host, route, at_ms };
                        let slot =
                            &mut state.last_read[usize::from(crate::wallet::chain::is_sepolia())];
                        newer = slot.map_or(true, |r| r.block != block || r.host != host);
                        *slot = Some(read);
                    }
                    apply(state, snap) | newer
                }
                None => {
                    // The link dropped; fall back to the diagnostic next refresh.
                    state.net.rpc_chain_ok = false;
                    state.status = b"reconnecting to the network";
                    true
                }
            }
        }
        Finished::Follow { followed, address, chain } => {
            let here = address == state.address && chain == crate::wallet::chain::current().id;
            let changed = match followed {
                Some(f) if here => followed_now(state, f),
                _ => false,
            };
            /* The account is read in the same refresh, as it always was. */
            state.net_job = Some(account_job(state));
            changed
        }
    }
}

/// Apply a snapshot, returning whether anything shown changed.
fn apply(state: &mut State, snap: Snapshot) -> bool {
    use super::reading::take;
    let staking = crate::wallet::chain::current().staking.is_some();
    let asked = if staking { 9 } else { 5 };
    let came = [
        snap.eth_balance.is_some(),
        snap.nonce.is_some(),
        snap.fee.is_some(),
        snap.nox_balance.is_some(),
        snap.usdc_balance.is_some(),
        snap.claimable.is_some(),
        snap.positions.is_some(),
        snap.passes.is_some(),
        snap.stats.is_some(),
    ]
    .iter()
    .filter(|c| **c)
    .count();
    if snap.nonce.is_some() && !state.nonce_ready {
        state.send_nonce = snap.nonce.unwrap_or(0);
    }
    let mut changed = take(snap.eth_balance, &mut state.balance_wei, &mut state.balance_ready);
    changed |= take(snap.nonce, &mut state.live_nonce, &mut state.nonce_ready);
    changed |= take(snap.fee, &mut state.fee_wei, &mut state.fee_ready);
    let nox = &mut state.nox;
    changed |= take(snap.nox_balance, &mut nox.balance_wei, &mut nox.balance_ready);
    changed |= take(snap.usdc_balance, &mut state.usdc_units, &mut state.usdc_ready);
    let nox = &mut state.nox;
    changed |= take(snap.claimable, &mut nox.claimable_wei, &mut nox.claimable_ready);
    changed |= take(snap.positions, &mut nox.positions, &mut nox.positions_ready);
    changed |= take(snap.passes, &mut nox.passes, &mut nox.passes_ready);
    let total = snap.stats.as_ref().map(|s| s.total);
    let rewards = snap.stats.as_ref().map(|s| s.rewards);
    let apr = snap.stats.as_ref().and_then(|s| s.apr);
    changed |= take(total, &mut nox.total_staked_wei, &mut nox.stats_ready);
    if let Some(r) = rewards {
        nox.rewards_distributed_wei = r;
    }
    /* A rate that cannot be worked out (nothing staked) is not the last one. */
    changed |= take(apr, &mut nox.apr_bps, &mut nox.apr_ready);
    let status: &'static [u8] = match came {
        0 => {
            /* An answer with no readings in it: the link is checked again. */
            state.net.rpc_chain_ok = false;
            b"the network answered without any readings, checking the link again"
        }
        n if n < asked => b"some readings did not come, shown as not read",
        _ => state.net.status,
    };
    changed |= status != state.status;
    state.status = status;
    changed
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

/// Kick a fresh refresh from the top on the next idle tick without blocking now.
pub fn probe_kick(state: &mut State) -> EventOutcome {
    state.probe_step = 1;
    EventOutcome::Repaint
}

/* Each payment's fate from one reading; the one on the Sent screen also
 * says it there. True when anything shown changed. */
fn followed_now(state: &mut State, f: crate::wallet::net::step::Followed) -> bool {
    use crate::wallet::send::sent::{Fate, Reading};
    let now = nonos_libc::mk_uptime_ms();
    let mut changed = false;
    for (hash, receipt) in f.receipts {
        let r = Reading { receipt, head: f.head, latest: f.latest, pending: f.pending };
        let Some(fate) = state.sent.judge(&hash, r, now) else { continue };
        changed = true;
        if hash != state.broadcast_hash {
            continue;
        }
        match fate {
            Fate::Confirmed { ok } => {
                crate::wallet::send::follow(state, &hash, Some(ok));
            }
            Fate::InBlock { .. } => {
                state.status = b"in a block, waiting for it to be confirmed";
            }
            Fate::Replaced => {
                state.receipt_ready = true;
                state.receipt_ok = false;
                state.broadcast_unknown = false;
                state.status = b"replaced: another transaction used its nonce";
            }
            Fate::Dropped => {
                state.receipt_ready = true;
                state.receipt_ok = false;
                state.broadcast_unknown = false;
                state.status = b"dropped: no block has it, nothing was paid";
            }
            Fate::Waiting => {
                state.status = b"sent, waiting for a block";
            }
        }
    }
    changed
}
