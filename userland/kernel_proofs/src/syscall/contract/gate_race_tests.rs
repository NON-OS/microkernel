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

//! The syscall gate against grant, revoke, thread join and teardown on other
//! threads. Real OS threads stand in for CPUs; the token MAC, the authority
//! cells, the epoch, mint, reissue, the snapshot and the resolver are the
//! kernel's. The model supplies only the process table and the lock order
//! `process::caps` uses around them.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use spin::{Mutex, RwLock};

use super::allowlist::Allowlist;
use super::args::SyscallArgs;
use super::cap_table::is_allowed;
use super::resolver::{resolve, ResolveContext};
use crate::capabilities::token::{sign_token, test_key};
use crate::capabilities::{caps_to_bits, Capability, CapabilityToken};
use crate::process::caps::authority::{snapshot, Cells};
use crate::process::caps::epoch::advance;
use crate::process::caps::mint::{mint, Binding};
use crate::process::caps::reissue::{reissue, Change, Member, Refusal};
use crate::syscall::abi::REGISTRY;
use crate::syscall::numbers::SyscallNumber;

const ASID: u32 = 7;
const NONCE: [u8; 16] = [9; 16];

struct Pcb {
    pid: u32,
    tgid: u32,
    token: RwLock<Arc<CapabilityToken>>,
    list: RwLock<Allowlist>,
    bits: AtomicU64,
    epoch: AtomicU64,
}

impl Pcb {
    fn cells(&self) -> Cells<'_> {
        Cells { token: &self.token, list: &self.list, bits: &self.bits, epoch: &self.epoch }
    }

    fn member(&self) -> Member<'_> {
        Member { cells: self.cells(), binding: binding(self.pid, 0) }
    }
}

fn binding(pid: u32, epoch: u64) -> Binding {
    Binding { pid, asid: ASID, epoch, boot_nonce: NONCE }
}

/// `build_pcb`: a block whose token, list and bits agree from the start.
fn build(pid: u32, tgid: u32, bits: u64) -> Arc<Pcb> {
    test_key();
    let token = mint(bits, binding(pid, 0)).expect("signing key set");
    Arc::new(Pcb {
        pid,
        tgid,
        list: RwLock::new(super::allowlist::for_token(&token)),
        token: RwLock::new(token),
        bits: AtomicU64::new(bits),
        epoch: AtomicU64::new(0),
    })
}

/// The process table and the mutator lock, as `process::caps` holds them.
struct Table {
    pcbs: RwLock<Vec<Arc<Pcb>>>,
    mutate: Mutex<()>,
}

impl Table {
    fn new() -> Self {
        Table { pcbs: RwLock::new(Vec::new()), mutate: Mutex::new(()) }
    }

    fn add(&self, p: Arc<Pcb>) {
        self.pcbs.write().push(p);
    }

    fn find(&self, pid: u32) -> Option<Arc<Pcb>> {
        self.pcbs.read().iter().find(|p| p.pid == pid).cloned()
    }

    fn group(&self, pid: u32) -> Vec<Arc<Pcb>> {
        let Some(target) = self.find(pid) else { return Vec::new() };
        let mut all: Vec<_> =
            self.pcbs.read().iter().filter(|p| p.pid != pid && p.tgid == target.tgid).cloned().collect();
        all.insert(0, target);
        all
    }

    fn change(&self, pid: u32, change: Change) -> Result<(), Refusal> {
        let _held = self.mutate.lock();
        let pcbs = self.group(pid);
        let from = pcbs[0].bits.load(Ordering::Acquire);
        let members: Vec<Member> = pcbs.iter().map(|p| p.member()).collect();
        reissue(&members, from, change)
    }

    fn grant(&self, pid: u32, mask: u64) {
        self.change(pid, Change::Grant(mask)).expect("grant");
    }

    fn revoke(&self, pid: u32, mask: u64) -> Result<(), Refusal> {
        self.change(pid, Change::Revoke(mask))
    }

    /// `terminate_process`: out of the table.
    fn teardown(&self, pid: u32) {
        self.pcbs.write().retain(|p| p.pid != pid);
    }

    /// `spawn_thread_parked` then `join_group`.
    fn spawn_thread(&self, parent: u32, tid: u32) -> Arc<Pcb> {
        let p = self.find(parent).expect("parent");
        let pcb = build(tid, p.tgid, p.bits.load(Ordering::Acquire));
        self.add(pcb.clone());
        let _held = self.mutate.lock();
        let peer = self
            .pcbs
            .read()
            .iter()
            .filter(|q| q.pid != tid && q.tgid == pcb.tgid)
            .min_by_key(|q| q.pid != q.tgid)
            .cloned()
            .expect("peer");
        let bits = peer.bits.load(Ordering::Acquire);
        if bits != pcb.bits.load(Ordering::Acquire) {
            reissue(&[pcb.member()], bits, Change::Set(bits)).expect("join");
        }
        pcb
    }
}

fn never() -> bool {
    false
}

/// `Capability::resolve`: the block, its snapshot, the resolver.
fn gate(table: &Table, pid: u32, number: SyscallNumber) -> bool {
    let Some(pcb) = table.find(pid) else { return false };
    let a = snapshot(&pcb.cells());
    let ctx = ResolveContext {
        current_asid: ASID,
        boot_session_nonce: Some(NONCE),
        capsule_revocation_epoch: a.epoch,
        caller_launches: never,
        allowlist: a.list,
    };
    resolve(&a.token, number, &SyscallArgs::new([0; 6]), &ctx).is_ok()
}

/// A syscall the tables allow with `cap` alone and with every capability,
/// and refuse with every capability but `cap`.
fn needing(cap: Capability) -> Option<SyscallNumber> {
    let alone = CapabilityToken::with_caps(&[cap]);
    let all = CapabilityToken::with_caps(Capability::all());
    let rest: Vec<Capability> = Capability::all().iter().copied().filter(|c| *c != cap).collect();
    let without = CapabilityToken::with_caps(&rest);
    REGISTRY
        .iter()
        .flat_map(|s| s.iter())
        .find(|e| {
            is_allowed(&alone, e.variant, false)
                && is_allowed(&all, e.variant, false)
                && !is_allowed(&without, e.variant, false)
        })
        .map(|e| e.variant)
}

fn bit(cap: Capability) -> u64 {
    caps_to_bits(&[cap])
}

/// Two capabilities, each with a syscall that needs it and not the other.
fn two() -> ((Capability, SyscallNumber), (Capability, SyscallNumber)) {
    let mut found = Capability::all().iter().filter_map(|c| needing(*c).map(|n| (*c, n)));
    let first = found.next().expect("a capability some syscall needs");
    let second = found.next().expect("a second one");
    (first, second)
}

const ROUNDS: usize = 300;

/*
 * The caller reads `done` before each call. A call that began after it saw
 * the revoke return must be refused; one that began before may go either
 * way, which is the call linearising before the revoke.
 */
fn caller_after(table: &Table, pid: u32, number: SyscallNumber, done: &AtomicBool) -> usize {
    let mut after = 0;
    loop {
        let seen = done.load(Ordering::Acquire);
        let ok = gate(table, pid, number);
        if seen {
            assert!(!ok, "pid {pid}: {number:?} allowed after the change returned");
            after += 1;
            if after == 64 {
                return after;
            }
        }
    }
}

#[test]
fn a_revoke_on_one_cpu_denies_the_next_call_on_another() {
    let ((cap, number), _) = two();
    for round in 0..ROUNDS {
        let table = Arc::new(Table::new());
        let pid = 100 + round as u32;
        table.add(build(pid, pid, caps_to_bits(Capability::all())));
        assert!(gate(&table, pid, number));
        let done = Arc::new(AtomicBool::new(false));
        let start = Arc::new(Barrier::new(2));
        let caller = {
            let (table, done, start) = (table.clone(), done.clone(), start.clone());
            thread::spawn(move || {
                start.wait();
                caller_after(&table, pid, number, &done)
            })
        };
        start.wait();
        table.revoke(pid, bit(cap)).expect("revoke");
        done.store(true, Ordering::Release);
        caller.join().expect("caller");
    }
}

#[test]
fn a_revoke_reaches_every_thread_of_the_process() {
    let ((cap, number), _) = two();
    for round in 0..ROUNDS {
        let table = Arc::new(Table::new());
        let pid = 1000 + 10 * round as u32;
        table.add(build(pid, pid, caps_to_bits(Capability::all())));
        let tids: Vec<u32> = (1..=3).map(|i| pid + i).collect();
        for &t in &tids {
            table.spawn_thread(pid, t);
            assert!(gate(&table, t, number));
        }
        let done = Arc::new(AtomicBool::new(false));
        let start = Arc::new(Barrier::new(tids.len() + 1));
        let callers: Vec<_> = tids
            .iter()
            .map(|&t| {
                let (table, done, start) = (table.clone(), done.clone(), start.clone());
                thread::spawn(move || {
                    start.wait();
                    caller_after(&table, t, number, &done)
                })
            })
            .collect();
        start.wait();
        // Asked of one thread; every thread and the leader lose it.
        table.revoke(tids[1], bit(cap)).expect("revoke");
        done.store(true, Ordering::Release);
        for c in callers {
            c.join().expect("caller");
        }
        assert!(!gate(&table, pid, number));
    }
}

#[test]
fn grants_racing_calls_never_allow_what_was_never_granted() {
    let ((granted, g_number), (withheld, w_number)) = two();
    let table = Arc::new(Table::new());
    let pid = 7000;
    table.add(build(pid, pid, 0));
    let stop = Arc::new(AtomicBool::new(false));
    let caller = {
        let (table, stop) = (table.clone(), stop.clone());
        thread::spawn(move || {
            while !stop.load(Ordering::Acquire) {
                assert!(!gate(&table, pid, w_number), "{w_number:?} allowed, never granted");
                gate(&table, pid, g_number);
            }
        })
    };
    for _ in 0..2000 {
        table.grant(pid, bit(granted));
        assert!(gate(&table, pid, g_number), "{g_number:?} refused after the grant returned");
        table.revoke(pid, bit(granted)).expect("revoke");
        assert!(!gate(&table, pid, g_number), "{g_number:?} allowed after the revoke returned");
    }
    stop.store(true, Ordering::Release);
    caller.join().expect("caller");
    assert_ne!(bit(withheld) & table.find(pid).expect("pcb").bits.load(Ordering::Acquire), bit(withheld));
}

#[test]
fn a_grant_racing_a_revoke_never_reinstates_what_was_revoked() {
    let ((kept, _), (taken, t_number)) = two();
    for round in 0..ROUNDS {
        let table = Arc::new(Table::new());
        let pid = 8000 + round as u32;
        table.add(build(pid, pid, bit(taken)));
        let start = Arc::new(Barrier::new(2));
        let granter = {
            let (table, start) = (table.clone(), start.clone());
            thread::spawn(move || {
                start.wait();
                for _ in 0..8 {
                    table.grant(pid, bit(kept));
                }
            })
        };
        start.wait();
        table.revoke(pid, bit(taken)).expect("revoke");
        granter.join().expect("granter");
        let pcb = table.find(pid).expect("pcb");
        assert_eq!(pcb.bits.load(Ordering::Acquire) & bit(taken), 0);
        assert!(!pcb.token.read().permissions.contains(&taken));
        assert!(!gate(&table, pid, t_number));
    }
}

#[test]
fn a_thread_spawned_during_a_revoke_holds_the_reduced_set() {
    let ((cap, number), _) = two();
    for round in 0..ROUNDS {
        let table = Arc::new(Table::new());
        let pid = 20_000 + 10 * round as u32;
        table.add(build(pid, pid, caps_to_bits(Capability::all())));
        let start = Arc::new(Barrier::new(2));
        let spawner = {
            let (table, start) = (table.clone(), start.clone());
            thread::spawn(move || {
                start.wait();
                table.spawn_thread(pid, pid + 1);
            })
        };
        start.wait();
        table.revoke(pid, bit(cap)).expect("revoke");
        spawner.join().expect("spawner");
        assert!(!gate(&table, pid + 1, number), "round {round}: new thread kept the revoked set");
        assert!(!gate(&table, pid, number));
    }
}

#[test]
fn teardown_racing_calls_denies_every_call_after_it() {
    let ((_, number), _) = two();
    for round in 0..ROUNDS {
        let table = Arc::new(Table::new());
        let pid = 40_000 + round as u32;
        table.add(build(pid, pid, caps_to_bits(Capability::all())));
        let done = Arc::new(AtomicBool::new(false));
        let start = Arc::new(Barrier::new(2));
        let caller = {
            let (table, done, start) = (table.clone(), done.clone(), start.clone());
            thread::spawn(move || {
                start.wait();
                caller_after(&table, pid, number, &done)
            })
        };
        start.wait();
        table.teardown(pid);
        done.store(true, Ordering::Release);
        caller.join().expect("caller");
    }
}

#[test]
fn a_token_older_than_the_epoch_is_refused() {
    let ((_, number), _) = two();
    let table = Table::new();
    table.add(build(1, 1, caps_to_bits(Capability::all())));
    let pcb = table.find(1).expect("pcb");
    assert!(gate(&table, 1, number));
    // The advance a revoke makes before it publishes, seen alone.
    advance(&pcb.epoch).expect("advance");
    assert!(!gate(&table, 1, number));
}

#[test]
fn the_epoch_stops_at_its_top() {
    let e = AtomicU64::new(u64::MAX - 1);
    assert_eq!(advance(&e), Some(u64::MAX));
    assert_eq!(advance(&e), None);
    assert_eq!(e.load(Ordering::Acquire), u64::MAX);
    assert_eq!(advance(&e), None);
    assert_eq!(e.load(Ordering::Acquire), u64::MAX);
}

#[test]
fn racing_advances_near_the_top_hand_out_each_epoch_once() {
    let e = Arc::new(AtomicU64::new(u64::MAX - 100));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let e = e.clone();
            thread::spawn(move || (0..50).filter_map(|_| advance(&e)).collect::<Vec<u64>>())
        })
        .collect();
    let mut got: Vec<u64> = handles.into_iter().flat_map(|h| h.join().expect("advancer")).collect();
    got.sort_unstable();
    let want: Vec<u64> = (u64::MAX - 99..=u64::MAX).collect();
    assert_eq!(got, want);
}

#[test]
fn a_revoke_at_the_top_withdraws_everything() {
    let ((cap, number), (_, other)) = two();
    let table = Table::new();
    table.add(build(1, 1, caps_to_bits(Capability::all())));
    let pcb = table.find(1).expect("pcb");
    pcb.epoch.store(u64::MAX, Ordering::Release);
    assert_eq!(table.revoke(1, bit(cap)), Err(Refusal::EpochExhausted));
    assert_eq!(pcb.bits.load(Ordering::Acquire), 0);
    assert!(pcb.token.read().permissions.is_empty());
    let empty = mint(0, binding(1, u64::MAX)).expect("mint");
    assert_eq!(*pcb.list.read(), super::allowlist::for_token(&empty));
    assert!(!gate(&table, 1, number));
    assert!(!gate(&table, 1, other));
    // Further revokes stay refused and never wrap.
    assert_eq!(table.revoke(1, 0), Err(Refusal::EpochExhausted));
    assert_eq!(pcb.epoch.load(Ordering::Acquire), u64::MAX);
}

#[test]
fn a_token_altered_after_signing_gets_an_empty_list() {
    test_key();
    let mut t = CapabilityToken::with_caps(Capability::all());
    sign_token(&mut t).expect("sign");
    assert_ne!(super::allowlist::for_token(&t), Allowlist::EMPTY);
    t.permissions.pop();
    assert_eq!(super::allowlist::for_token(&t), Allowlist::EMPTY);
}

#[test]
fn a_nonce_drawn_without_randomness_is_an_error_not_zeros() {
    // The host stands in for a machine whose RNG does not answer.
    assert!(crate::capabilities::token::secure_nonce().is_err());
}
