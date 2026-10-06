//! The wallet the shells hold. Its session sits behind a lock, so any thread can call it.

mod access;
mod account;
mod account_confirm;
mod account_send;
mod account_swap;
mod account_view;
mod accounts;
mod chain;
mod deposit;
mod export;
mod fee;
mod find;
mod find_steps;
mod follow;
mod history;
mod import;
mod lifecycle;
mod measure;
mod new;
mod own;
mod pending;
mod pending_batch;
mod pool_read;
mod profile;
mod publish;
mod publish_view;
mod quote;
mod quote_view;
mod relay;
mod rewards;
mod settle_self;
mod shield;
mod shield_split;
mod shield_view;
mod spend;
mod swap_rate;
mod swap_view;
mod sync_each;
mod take_back;
mod ticket;
mod watching;
mod wipe;
mod withdraw_to;
mod words;

use crate::custody::HardwareGuard;
use crate::net::tor::Tor;
use crate::wallet::{History, Paths, Session};
use pending::Pending;
use pending_batch::PendingBatch;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex};

pub struct Wallet {
    paths: Paths,
    guard: Arc<dyn HardwareGuard>,
    session: Mutex<Option<Session>>,
    tor: Mutex<Option<Arc<Tor>>>,
    pending: Mutex<Option<Pending>>,
    quoted: Mutex<Option<quote::Quoted>>,
    batch: Mutex<Option<PendingBatch>>,
    /// The review that settles a kept spend from the owner's account, and that spend's hand-off:
    /// once it is confirmed the hand-off is marked, so no lander is handed it again.
    settles: Mutex<Option<(u64, std::path::PathBuf)>>,
    reviews: AtomicU64,
    /// The active account, for the Tor client to route by without taking the session lock.
    active: AtomicU32,
    searched: AtomicU32,
    /// How many times the reads of the pool under way were told to stop, as by a lock.
    stops: AtomicU64,
    /// The pool's history as last read, for the next read to go on from.
    read: Mutex<Option<Arc<History>>>,
}
