//! What the shells' wallet holds: its paths and guard, the session, and the reviews in flight.

use super::pending::Pending;
use super::pending_batch::PendingBatch;
use super::quote;
use crate::custody::HardwareGuard;
use crate::net::tor::Tor;
use crate::wallet::{History, Paths, Session};
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex};

pub struct Wallet {
    pub(super) paths: Paths,
    pub(super) guard: Arc<dyn HardwareGuard>,
    pub(super) session: Mutex<Option<Session>>,
    pub(super) tor: Mutex<Option<Arc<Tor>>>,
    pub(super) pending: Mutex<Option<Pending>>,
    pub(super) quoted: Mutex<Option<quote::Quoted>>,
    pub(super) batch: Mutex<Option<PendingBatch>>,
    /// The review that settles a kept spend from the owner's account, and that spend's hand-off:
    /// once it is confirmed the hand-off is marked, so no lander is handed it again.
    pub(super) settles: Mutex<Option<(u64, std::path::PathBuf)>>,
    pub(super) reviews: AtomicU64,
    /// The active account, for the Tor client to route by without taking the session lock.
    pub(super) active: AtomicU32,
    pub(super) searched: AtomicU32,
    /// How many times the reads of the pool under way were told to stop, as by a lock.
    pub(super) stops: AtomicU64,
    /// The pool's history as last read, for the next read to go on from.
    pub(super) read: Mutex<Option<Arc<History>>>,
}
