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
 * A finished job's answer, put where the screens read it. Each op has one
 * place here; an answer the service did not give is never filled in.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use crate::wallet::state::shield_log::Kind;
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

fn text(body: &str, name: &str) -> String {
    String::from(field(body, name).unwrap_or(""))
}

fn number(body: &str, name: &str) -> Option<u32> {
    field(body, name)?.parse().ok()
}

/* The service's own state: the receive address and the shielded holdings. */
pub fn state(state: &mut State, body: &str) {
    let ui = &mut state.shield_ui;
    if let Some(a) = field(body, "address") {
        ui.nox1 = Some(String::from(a));
    }
    /* Read again from a service that already held the store. */
    if let Some(kept) = field(body, "kept") {
        ui.live = kept == "memory";
    }
    if let Some(history) = super::reply::entries(body) {
        ui.history = history;
        ui.history_cut = field(body, "history_cut").and_then(|n| n.parse().ok()).unwrap_or(0);
    }
    /* What the store would not give is said, and what was shown stays. */
    let unread = [("balances_why", "balances"), ("history_why", "history")];
    for (name, what) in unread {
        if let Some(why) = field(body, name) {
            ui.failure = Some(format!("The shielded {what} could not be read: {why}"));
        }
    }
    if field(body, "balances") != Some("1") {
        return;
    }
    let mut held = [None, None];
    for line in fields(body, "balance") {
        let parts: Vec<&str> = line.split(' ').collect();
        let [coin, spendable, pending, _notes] = parts[..] else { continue };
        let slot = match coin {
            "ETH" => 0,
            "NOX" => 1,
            _ => continue,
        };
        held[slot] =
            Some(Held { spendable: String::from(spendable), pending: String::from(pending) });
    }
    ui.held = held;
}

/* The service's state read again after a job. One that did not answer is
 * said, and what was shown stays until the next read. */
fn refresh(state: &mut State) {
    match super::client::call(OP_STATE, &[]) {
        Ok(a) if a.status == STATUS_OK => self::state(state, &a.body),
        Ok(a) => {
            let why = field(&a.body, "why").unwrap_or(super::client::NO_ANSWER);
            state.shield_ui.failure =
                Some(format!("The shield's state could not be read again: {why}"));
        }
        Err(why) => {
            state.shield_ui.failure =
                Some(format!("The shield's state could not be read again: {why}"));
        }
    }
}

pub fn apply(state: &mut State, op: u16, body: &str) {
    match op {
        OP_OPEN_WORDS | OP_OPEN_KEY => {
            state.shield_ui.opened = true;
            state.shield_ui.phase = None;
            /* How long each step of the open took, for whoever checks a boot. */
            if let Some(took) = field(body, "took") {
                state.shield_ui.news = Some(format!("The shield opened ({took})."));
            }
            state.shield_ui.nox1 = field(body, "address").map(String::from);
            state.shield_ui.live = field(body, "kept") == Some("memory");
            super::job::start(state, OP_SYNC, &[]);
        }
        OP_SYNC => {
            let ui = &mut state.shield_ui;
            ui.last_sync_ms = mk_uptime_ms();
            if let (Some(l), Some(m)) = (number(body, "wait_leaves"), number(body, "wait_minutes"))
            {
                ui.wait_left = Some((l, m));
            }
            refresh(state);
        }
        OP_REVIEW_SHIELD | OP_REVIEW_SELF_SETTLE => reviewed(state, op, body),
        OP_CONFIRM => confirmed(state, body),
        OP_QUOTE => quoted(state, body),
        OP_SEND | OP_WITHDRAW => proved(state, op, body),
        OP_FOLLOW => {
            super::follow::followed(state, body);
            refresh(state);
        }
        OP_TAKE_BACK => {
            let n = text(body, "returned");
            let ui = &mut state.shield_ui;
            /* Every spend's notes came back, so none of them goes on. */
            ui.spend = None;
            ui.earlier.clear();
            ui.news = Some(format!("{n} notes are spendable again. Nothing was paid."));
            refresh(state);
            super::job::start(state, OP_SYNC, &[]);
        }
        _ => {}
    }
}

pub fn failed(state: &mut State, op: u16, why: String) {
    let ui = &mut state.shield_ui;
    if matches!(op, OP_OPEN_WORDS | OP_OPEN_KEY) {
        ui.opened = false;
    }
    if matches!(op, OP_SEND | OP_WITHDRAW) {
        ui.proving = None;
        if ui.screen == SHIELD_PROVING {
            ui.screen = ui.from;
        }
    }
    ui.failure = Some(why);
}

fn reviewed(state: &mut State, op: u16, body: &str) {
    let ui = &mut state.shield_ui;
    if let Some(why) = field(body, "refusal") {
        ui.failure = Some(String::from(why));
        return;
    }
    ui.review = Some(Review {
        id: text(body, "id"),
        approval: field(body, "approval") == Some("1"),
        amount: text(body, "amount"),
        pool_fee: text(body, "pool_fee"),
        shielded: text(body, "shielded"),
        max_network_fee: text(body, "max_network_fee"),
        valid_for: text(body, "valid_for"),
    });
    ui.from = if op == OP_REVIEW_SELF_SETTLE { SHIELD_SETTLE } else { SHIELD_DEPOSIT };
    ui.screen = SHIELD_REVIEW;
    state.scroll = 0;
}

fn confirmed(state: &mut State, body: &str) {
    let Some(review) = state.shield_ui.review.take() else { return };
    let settle = state.shield_ui.from == SHIELD_SETTLE;
    let kind = match (settle, review.approval) {
        (true, _) => Kind::SelfSettle,
        (_, true) => Kind::Approval,
        _ => Kind::Deposit,
    };
    let tx = field(body, "tx").map(String::from);
    refresh(state);
    let ui = &mut state.shield_ui;
    /* The transaction is named when the service named it. */
    let hash = tx.map(|t| format!(" ({t})")).unwrap_or_default();
    ui.news = Some(match kind {
        Kind::Approval => {
            format!("Approval sent{hash}. Once it lands, review the deposit again to send it.")
        }
        Kind::SelfSettle => {
            let id = ui.settling_id.take();
            let spend = match &id {
                None => ui.spend.as_mut(),
                Some(id) => ui.earlier.iter_mut().find(|s| s.id.as_ref() == Some(id)),
            };
            if let Some(spend) = spend {
                spend.state = String::from(super::follow::SETTLING);
                spend.self_settle = false;
            }
            /* Followed from now on to the block, as a lander's would be. */
            ui.last_follow_ms = 0;
            format!("Sent from your own account{hash}. It links this account to the payment.")
        }
        _ => format!("Deposit sent{hash}. It is spendable once it matures in the pool."),
    });
    ui.screen = if settle { SHIELD_PROVING } else { SHIELD_HOME };
    ui.size = None;
    state.scroll = 0;
}

fn quoted(state: &mut State, body: &str) {
    let ui = &mut state.shield_ui;
    if let Some(why) = field(body, "refusal") {
        ui.failure = Some(String::from(why));
        return;
    }
    ui.quote = Some(Quote {
        network_fee: text(body, "network_fee"),
        protocol_fee: text(body, "protocol_fee"),
        total_fee: text(body, "total_fee"),
    });
    ui.from = ui.screen;
    ui.screen = SHIELD_REVIEW;
    state.scroll = 0;
}

fn proved(state: &mut State, op: u16, body: &str) {
    let kind = if op == OP_SEND { Kind::Send } else { Kind::Withdraw };
    let published = super::reply::taken(body);
    /* What was confirmed, not what the form holds now. */
    let (asset, amount) = state
        .shield_ui
        .proving
        .take()
        .unwrap_or_else(|| (state.shield_ui.asset, super::actions::amount(&state.shield_ui)));
    refresh(state);
    let ui = &mut state.shield_ui;
    /* The last spend may still be on its way: the service keeps it, and it
     * is followed among the earlier ones. */
    if let Some(last) = ui.spend.take().filter(|s| s.tx.is_none()) {
        ui.earlier.push(last);
    }
    ui.spend = Some(Spend {
        id: None,
        kind,
        asset,
        amount,
        published,
        state: String::from(if published {
            "handed to a lander"
        } else {
            "kept, no lander took it"
        }),
        minutes: 0,
        tx: None,
        self_settle: !published,
        refusal: field(body, "lander_refusal").map(String::from),
        weakened: fields(body, "weakened").map(String::from).collect(),
    });
    ui.last_follow_ms = mk_uptime_ms();
    ui.quote = None;
    ui.to.clear();
    ui.amount.clear();
    ui.override_text.clear();
    ui.screen = SHIELD_PROVING;
}
