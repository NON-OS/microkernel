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

//! Each call the wallet makes, on the phones' wallet object.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use nox_shield_core::ffi::{
    address_of_key, address_of_words, format_amount, CancelToken, PublicShield, Wallet,
};
use nox_shield_core::net::asset::Coin;
use shield_wire::{Values, BODY_MAX};

use crate::guard::MachineGuard;
use crate::jobs;

/// One store per 0x account, so a second wallet on this machine never
/// opens, or overwrites, the first one's notes.
///
/// Kept in memory on every boot. The store is files the phones' core writes
/// with std::fs, and vfs keeps only what a capsule hands to OP_STORE_PERSIST,
/// one fixed record at a time: these files are in its RAM tree and gone at
/// power off, whatever volume is mounted. So the store says so (`kept` is
/// "memory") and its file key is wrapped under a key drawn for this boot,
/// never under the machine key, which would claim a keeping it does not
/// have. The words open it again on the next boot, and a sync finds the
/// notes on chain.
const ROOT_LIVE: &str = "/run/shield";

struct Open {
    address: String,
    dir: PathBuf,
    wallet: Arc<Wallet>,
    live: bool,
}

static OPEN: Mutex<Option<Open>> = Mutex::new(None);
/// Counts locks. An open that began before the last lock does not become
/// the open store when it ends: the wallet locked while it ran.
static LOCKS: AtomicU64 = AtomicU64::new(0);
static CANCEL: Mutex<Option<Arc<CancelToken>>> = Mutex::new(None);

/// What a state request answers, taken by the worker after each job. A
/// sync or a proof holds the store for minutes; the answer never waits on it.
struct Seen {
    account: String,
    wallet: Arc<Wallet>,
    address: Option<String>,
    /// One line a coin, or why the store would not give them.
    balances: Result<Vec<String>, String>,
    live: bool,
    /// Where a withdrawal pays, read here so asking for it never waits on
    /// the store a proof holds. None for a wallet from a key; an error is
    /// the store's, never taken for that.
    fresh: Result<Option<String>, String>,
    /// The account's history from its store, one line an entry, oldest
    /// first, or why the store would not give it.
    history: Result<Vec<String>, String>,
}

static SEEN: Mutex<Option<Seen>> = Mutex::new(None);

pub type Answer = Result<Values, String>;

fn refuse<T>(why: impl core::fmt::Display) -> Result<T, String> {
    Err(why.to_string())
}

fn wallet() -> Result<Arc<Wallet>, String> {
    let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
    open.as_ref()
        .map(|o| Arc::clone(&o.wallet))
        .ok_or_else(|| String::from("Open the shield from the wallet first."))
}

/// The open wallet, made ready to prove: a thread for the prover, and the
/// shipped periodic cache in its folder.
fn prover() -> Result<Arc<Wallet>, String> {
    if crate::pool::workers() == 0 {
        return refuse("No thread could be started for the prover.");
    }
    /* The folder and the wallet are taken and the lock let go before the cache is written:
     * an 8 MiB write under OPEN would hold the service loop past the wallet's timeout. */
    let (dir, wallet) = {
        let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
        let o =
            open.as_ref().ok_or_else(|| String::from("Open the shield from the wallet first."))?;
        (o.dir.clone(), Arc::clone(&o.wallet))
    };
    crate::periodic::seed(&dir);
    Ok(wallet)
}

pub fn coin(text: &str) -> Result<Coin, String> {
    match text {
        "ETH" => Ok(Coin::Eth),
        "NOX" => Ok(Coin::Nox),
        _ => refuse("The shield holds ETH and NOX only."),
    }
}

fn flag(text: &str) -> bool {
    text == "1"
}

fn dir_of(address: &str) -> Result<String, String> {
    let hex = address.trim().trim_start_matches("0x").to_ascii_lowercase();
    if hex.len() != 40 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return refuse("The wallet named no account.");
    }
    Ok(format!("{ROOT_LIVE}/{hex}"))
}

/// Open the shield of the 0x account `address`: unlock the store it has, or
/// make it from `open` once. The account the store opens must be `address`.
fn open_with(address: &str, make: impl FnOnce(&Wallet) -> Result<(), String>) -> Answer {
    let locks = LOCKS.load(Ordering::SeqCst);
    let dir = dir_of(address)?;
    crate::steps::step("making the store's folder");
    std::fs::create_dir_all(&dir).map_err(|_| {
        String::from("This machine will not keep a shield store, not even in memory.")
    })?;
    let live = true;
    let new =
        || Wallet::new(dir.clone(), Arc::new(MachineGuard { live })).map_err(|e| e.to_string());
    crate::steps::step("opening the store");
    let mut wallet = new()?;
    let stored = wallet.stored();
    crate::steps::step(if stored { "unlocking the store" } else { "restoring from the words" });
    match (stored, live) {
        (false, _) => make(&wallet)?,
        (true, false) => wallet.unlock().map_err(|e| e.to_string())?,
        (true, true) => {
            // Kept in memory by a service that has since restarted: its key went
            // with it, so the store is made again from the words.
            if wallet.unlock().is_err() {
                drop(wallet);
                let _ = std::fs::remove_dir_all(&dir);
                std::fs::create_dir_all(&dir)
                    .map_err(|_| String::from("This machine will not keep a shield store."))?;
                wallet = new()?;
                make(&wallet)?;
            }
        }
    }
    crate::steps::step("choosing the account");
    let opened = match pick_account(&wallet, address.trim()) {
        Ok(a) => a,
        Err(why) => {
            let _ = wallet.lock();
            return Err(why);
        }
    };
    crate::steps::step("reading the private address");
    let mut v = Values::new();
    v.put("address", &wallet.receiving_address().map_err(|e| e.to_string())?);
    v.put("kept", if live { "memory" } else { "volume" });
    v.put("took", &crate::steps::end());
    let mut open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
    if LOCKS.load(Ordering::SeqCst) != locks {
        drop(open);
        let _ = wallet.lock();
        return refuse("The wallet locked while the shield opened. Nothing is open.");
    }
    *open = Some(Open { address: opened, dir: PathBuf::from(dir), wallet, live });
    Ok(v)
}

/* The wallet's further accounts are m/44'/60'/0'/0/i of the same words, as
 * shield-core's are: make the one the wallet named the active account,
 * adding accounts up to it. A store whose accounts never reach it belongs
 * to another phrase. */
const MAX_ACCOUNTS: usize = 8;

fn pick_account(wallet: &Wallet, address: &str) -> Result<String, String> {
    for _ in 0..MAX_ACCOUNTS {
        let all = wallet.accounts().map_err(|e| e.to_string())?;
        if let Some(a) = all.iter().find(|a| a.public_address.eq_ignore_ascii_case(address)) {
            if !a.active {
                wallet.select_account(a.index).map_err(|e| e.to_string())?;
            }
            return Ok(a.public_address.clone());
        }
        if all.len() >= MAX_ACCOUNTS || wallet.from_key().unwrap_or(true) {
            break;
        }
        wallet.add_account().map_err(|e| e.to_string())?;
    }
    refuse("The shield store here belongs to another account. Nothing was opened.")
}

pub fn open_words(f: &[&str]) -> Answer {
    let [address, words] = f else { return refuse("Name the account and its words.") };
    let list: Vec<String> = words.split(' ').filter(|w| !w.is_empty()).map(String::from).collect();
    /* The private address first, from the words alone: shown before the store exists. */
    crate::steps::begin();
    crate::steps::step("deriving the private address");
    if let Some(account) = public(address) {
        crate::steps::preview(address_of_words(&list, account).ok().flatten());
    }
    open_with(address, |w| w.restore(list).map_err(|e| e.to_string()))
}

pub fn open_key(f: &[&str]) -> Answer {
    let [address, key] = f else { return refuse("Name the account and its key.") };
    let key = String::from(*key);
    crate::steps::begin();
    crate::steps::step("deriving the private address");
    if let Some(account) = public(address) {
        crate::steps::preview(address_of_key(&key, account).ok().flatten());
    }
    open_with(address, move |w| w.restore_from_key(key).map_err(|e| e.to_string()))
}

/// The 0x account the wallet named, as bytes.
fn public(address: &str) -> Option<[u8; 20]> {
    let hex = address.trim().trim_start_matches("0x");
    if hex.len() != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

/// Answered at once, on the service loop: the store stops being the open
/// one now, a proof under way is stopped, and its keys are dropped off the
/// loop, since a proof or a scan may hold the store for minutes yet.
pub fn lock() -> Answer {
    LOCKS.fetch_add(1, Ordering::SeqCst);
    let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?.take();
    if let Ok(mut seen) = SEEN.lock() {
        *seen = None;
    }
    if let Ok(c) = CANCEL.lock() {
        if let Some(t) = c.as_ref() {
            t.cancel();
        }
    }
    if let Some(open) = open {
        let wallet = Arc::clone(&open.wallet);
        let spawned = std::thread::Builder::new().name(String::from("lock")).spawn(move || {
            let _ = wallet.lock();
        });
        if spawned.is_err() {
            // No thread: the keys are dropped here, which may wait on the store.
            let _ = open.wallet.lock();
        }
    }
    Ok(Values::new())
}

/// Take what a state request answers from the open store. On the worker,
/// after a job: here it may wait for the store, the service loop never does.
pub fn remember() {
    // The guard goes at the end of this statement: OPEN is taken again below.
    let Some(open) = OPEN
        .lock()
        .ok()
        .map(|o| o.as_ref().map(|o| (o.address.clone(), Arc::clone(&o.wallet), o.live)))
    else {
        return;
    };
    let seen = open.map(|(account, wallet, live)| {
        let address = wallet.receiving_address().ok();
        let fresh = wallet.fresh_withdrawal_address().map_err(|e| e.to_string());
        let history = wallet.history().map_err(|e| e.to_string()).map(|all| {
            all.iter()
                .map(|h| {
                    let tx = h.tx.as_deref().unwrap_or("");
                    crate::reply::entry(&h.kind, &h.coin, &h.amount, &h.stage, tx, h.at)
                })
                .collect()
        });
        let balances = wallet.balances().map_err(|e| e.to_string()).map(|all| {
            all.iter()
                .map(|b| {
                    let name = format!("{:?}", b.coin).to_uppercase();
                    format!(
                        "{name} {} {} {}",
                        format_amount(b.coin, b.spendable),
                        format_amount(b.coin, b.pending),
                        b.note_count
                    )
                })
                .collect()
        });
        Seen { account, wallet, address, balances, live, fresh, history }
    });
    let Ok(mut slot) = SEEN.lock() else { return };
    // Locked while this ran: what was read belongs to a store no longer open.
    let still = OPEN.lock().ok().and_then(|o| o.as_ref().map(|o| Arc::clone(&o.wallet)));
    *slot = seen.filter(|s| still.is_some_and(|w| Arc::ptr_eq(&w, &s.wallet)));
}

pub fn state() -> Answer {
    let mut v = Values::new();
    /* The running job goes first: it is what the wallet waits on, and
     * nothing after it can push it out of the reply. */
    if let Some(job) = jobs::running() {
        v.put("job", job);
    }
    let seen = SEEN.lock().map_err(|_| String::from("the shield is busy"))?;
    match seen.as_ref() {
        None => {
            v.put("unlocked", "0");
        }
        Some(s) => {
            v.put("unlocked", "1").put("account", &s.account);
            v.put("kept", if s.live { "memory" } else { "volume" });
            if let Some(a) = &s.address {
                v.put("address", a);
            }
            /* Only what was read is answered: a store that would not give
             * them says why, and the wallet keeps what it showed before. */
            match &s.balances {
                Ok(all) => {
                    v.put("balances", "1");
                    for b in all {
                        v.put("balance", b);
                    }
                }
                Err(why) => {
                    v.put("balances_why", why);
                }
            }
            /* The history goes last, its newest entries that fit the reply
             * whole; how many older ones were left out is said. */
            match &s.history {
                Ok(all) => {
                    v.put("history", "1");
                    let room = BODY_MAX.saturating_sub(v.text().len() + CUT_LINE);
                    let left = v.put_newest("entry", all, room);
                    if left > 0 {
                        v.put("history_cut", &left.to_string());
                    }
                }
                Err(why) => {
                    v.put("history_why", why);
                }
            }
        }
    }
    drop(seen);
    Ok(v)
}

/* Room kept for the `history_cut=<count>` line after the entries. */
const CUT_LINE: usize = 32;

fn shield_view(v: &mut Values, s: &PublicShield) {
    v.put("id", &s.id.to_string())
        .put("approval", if s.approval { "1" } else { "0" })
        .put("amount", &s.amount)
        .put("pool_fee", &s.pool_fee)
        .put("shielded", &s.shielded)
        .put("max_network_fee", &s.max_network_fee)
        .put("valid_for", &s.valid_for_seconds.to_string());
    if let Some(why) = &s.refusal {
        v.put("refusal", why);
    }
}

pub fn sync() -> Answer {
    let w = wallet()?;
    let s = w.sync_chain().map_err(scan_failed)?;
    let mut v = Values::new();
    v.put("head", &s.head.to_string())
        .put("received", &s.received.to_string())
        .put("deposited", &s.deposited.to_string())
        .put("spent", &s.spent.to_string())
        .put("wait_leaves", &s.wait_leaves.to_string())
        .put("wait_minutes", &s.wait_minutes.to_string());
    Ok(v)
}

pub fn review_shield(f: &[&str]) -> Answer {
    let [c, amount] = f else { return refuse("Name the coin and the amount.") };
    let review =
        wallet()?.review_shield(coin(c)?, String::from(*amount)).map_err(|e| e.to_string())?;
    let mut v = Values::new();
    shield_view(&mut v, &review);
    Ok(v)
}

pub fn confirm(f: &[&str]) -> Answer {
    let [id] = f else { return refuse("Name the review.") };
    let id: u64 = id.parse().map_err(|_| String::from("That review is not a number."))?;
    let sent = wallet()?.confirm_public_send(id).map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("tx", &sent.hash).put("link", &sent.link);
    Ok(v)
}

pub fn quote(f: &[&str]) -> Answer {
    let [c, amount, withdraw] = f else { return refuse("Name the coin, amount and kind.") };
    let q = wallet()?
        .quote_spend(coin(c)?, String::from(*amount), flag(withdraw))
        .map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("network_fee", &q.network_fee)
        .put("protocol_fee", &q.protocol_fee)
        .put("total_fee", &q.total_fee)
        .put("base_fee_gwei", &q.base_fee_gwei);
    if let Some(why) = &q.refusal {
        v.put("refusal", why);
    }
    Ok(v)
}

fn cancel_token() -> Result<Arc<CancelToken>, String> {
    let token = CancelToken::new();
    *CANCEL.lock().map_err(|_| String::from("the shield is busy"))? = Some(Arc::clone(&token));
    Ok(token)
}

/// The proof is over: its token goes, so the next spend never shows its progress.
fn proved() {
    if let Ok(mut c) = CANCEL.lock() {
        *c = None;
    }
}

/// After a spend is proved: hand it to a lander, and say what happened. A
/// lander that refused it, or none reached, is said with its reason, and
/// the owner may settle it from the public account.
fn publish(v: &mut Values, w: &Wallet) {
    match w.publish_spend() {
        Ok(p) => {
            v.put("published", "1").put("times", &p.times.to_string());
            if let Some(id) = &p.id {
                v.put("handoff", id);
            }
            if let Some(why) = &p.refusal {
                v.put("lander_refusal", why);
            }
        }
        Err(e) => {
            v.put("published", "0").put("lander_refusal", &e.to_string());
        }
    }
}

pub fn send(f: &[&str]) -> Answer {
    let [c, to, amount, early] = f else { return refuse("Name the coin, payee, amount and wait.") };
    let w = prover()?;
    let t = w.send_private(
        coin(c)?,
        String::from(*to),
        String::from(*amount),
        flag(early),
        cancel_token()?,
    );
    proved();
    let t = t.map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("proved", "1");
    for weak in &t.weakened {
        v.put("weakened", weak);
    }
    publish(&mut v, &w);
    Ok(v)
}

pub fn withdraw(f: &[&str]) -> Answer {
    let [c, to, amount, early] = f else {
        return refuse("Name the coin, address, amount and wait.");
    };
    let w = prover()?;
    let t = w.withdraw(
        coin(c)?,
        String::from(*to),
        String::from(*amount),
        flag(early),
        cancel_token()?,
    );
    proved();
    let t = t.map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("proved", "1");
    for weak in &t.weakened {
        v.put("weakened", weak);
    }
    publish(&mut v, &w);
    Ok(v)
}

/// One `earlier` value per spend kept before the last: its number, what became of it, minutes
/// since it was first published, whether its owner may settle it, and its settlement, with `|`
/// between them, since a state is words.
/// A shield_core follow as the reply builders take it.
macro_rules! followed {
    ($s:expr) => {
        crate::reply::Followed {
            state: &$s.state,
            minutes: $s.minutes_since_first,
            times: $s.times_published,
            self_settle: $s.self_settle_offered,
            tx: $s.tx.as_deref(),
            link: $s.link.as_deref(),
            refusal: $s.refusal.as_deref(),
        }
    };
}

fn follow_earlier(v: &mut Values, w: &Wallet) {
    match w.follow_earlier() {
        Ok(kept) => {
            for k in &kept {
                crate::reply::earlier(v, &k.id, &followed!(k.follow));
            }
        }
        Err(e) => {
            v.put("earlier_why", &e.to_string());
        }
    }
}

pub fn follow() -> Answer {
    let w = wallet()?;
    let mut v = Values::new();
    follow_earlier(&mut v, &w);
    let s = match w.follow_spend() {
        Ok(s) => s,
        // No record: nothing was published yet, so there is nothing to
        // follow. Try a lander again; until one takes it, the owner may
        // settle it. Any other error (the network, a locked store) is said
        // as it is, and the spend is followed again on the next ask.
        Err(nox_shield_core::error::WalletError::Unavailable) => {
            publish(&mut v, &w);
            let taken = shield_wire::field(v.text(), "published") == Some("1")
                && shield_wire::field(v.text(), "lander_refusal").is_none();
            v.put("state", if taken { "republished" } else { "unpublished" })
                .put("self_settle", if taken { "0" } else { "1" });
            return Ok(v);
        }
        Err(e) => return Err(scan_failed(e)),
    };
    crate::reply::follow(&mut v, &followed!(s));
    Ok(v)
}

/// The spend proved last, or with a field, the one kept under that number.
pub fn review_self_settle(f: &[&str]) -> Answer {
    let w = wallet()?;
    let review = match f {
        [] | [""] => w.review_self_settle(),
        [id] => w.review_self_settle_earlier(String::from(*id)),
        _ => return refuse("Name at most the payment to settle."),
    }
    .map_err(|e| e.to_string())?;
    let mut v = Values::new();
    shield_view(&mut v, &review);
    Ok(v)
}

pub fn take_back() -> Answer {
    let n = wallet()?.take_back_pending().map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("returned", &n.to_string());
    Ok(v)
}

/// Answered at once from what the worker read after the last job: the
/// store may be held by a proof, and the service loop never waits on it.
pub fn fresh_address() -> Answer {
    let seen = SEEN.lock().map_err(|_| String::from("the shield is busy"))?;
    let s = seen.as_ref().ok_or_else(|| String::from("Open the shield from the wallet first."))?;
    let mut v = Values::new();
    match &s.fresh {
        Ok(Some(a)) => {
            v.put("address", a);
        }
        Ok(None) => {}
        Err(why) => return Err(format!("the shield store could not name a fresh account: {why}")),
    }
    Ok(v)
}

/// How far the running spend's proof has come: the last phase the prover
/// finished, by name, and the fraction of the proof done then.
pub fn progress() -> Option<(String, f32)> {
    CANCEL.lock().ok()?.as_ref()?.progress()
}

pub fn cancel() -> Answer {
    if let Some(t) = CANCEL.lock().map_err(|_| String::from("the shield is busy"))?.as_ref() {
        t.cancel();
    }
    Ok(Values::new())
}

/// A scan's failure, with the server it asked and the step it stopped at, so the wallet says
/// exactly where reading the pool broke: the chain head, the leaf count, a history, the
/// registry. Said on the serial console too.
fn scan_failed(e: impl core::fmt::Display) -> String {
    let said = match nox_shield_core::net::rpc::progress::last_step() {
        Some((host, step)) => format!("{e}, while asking {host} for {step}"),
        None => e.to_string(),
    };
    crate::steps::serial(&format!("scan failed: {said}"));
    said
}
