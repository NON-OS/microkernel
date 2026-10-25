// NONOS Operating System (AGPL-3.0-or-later)
//! One shootdown round, the CPUs it must reach, and the APs that come up
//! while it runs, as a state machine small enough to search exhaustively.
//!
//! The model follows `src/memory/paging/manager/shootdown/` and the AP path
//! in `src/smp/init/ap_unit.rs` and `src/smp/ap/`. One kernel page changes
//! from `Old` to `New`. CPU 0 runs the round. Every other CPU is an AP that is
//! either online when the round starts or is brought up, or given up on,
//! while it runs. Each step is one atomic action of the code it names, and the
//! search takes every order the steps can run in.
//!
//! Memory is sequentially consistent here. The code makes it so with the
//! `SeqCst` fence before `select` and the one between setting a target's
//! pending flag and sending its IPI (`broadcast.rs`, `send.rs`), and with the
//! `SeqCst` stores of a descriptor's state.
//!
//! The hardware may cache: a CPU running kernel code may load the page's
//! translation into its TLB at any step and drop it at any step. INIT empties
//! a TLB (Intel SDM, vol. 3A, 9.1), so an AP woken by INIT and STARTUP starts
//! with nothing cached, and so does one the boot CPU gives up on and parks.

/// What a TLB entry for the page says, or what the page table says.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Val {
    Old,
    New,
}

/// How a target drops the page: `invlpg` for a range, or a CR3 reload for
/// `pages == 0` (`handle.rs`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flush {
    Page,
    Full,
}

/// One step of an AP's `bring_up`, as far as parking is concerned.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// The AP checks whether the boot CPU asked it to park, and parks there.
    Checkpoint,
    /// Takes a lock (the paging manager in `arm_ap_guards`, the allocator).
    Lock,
    /// Releases it.
    Unlock,
}

/// `bring_up` as the kernel has it: a checkpoint before and after each step
/// that takes a lock, and none inside one.
pub const BRING_UP: &[Step] = &[
    Step::Checkpoint,
    Step::Lock,
    Step::Unlock,
    Step::Checkpoint,
    Step::Lock,
    Step::Unlock,
    Step::Checkpoint,
];

/// The rules under test. `Config::kernel()` is the code; every other value is
/// a mutant that removes or reorders one rule.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Number of CPUs, the initiator included.
    pub cpus: usize,
    /// APs already online when the round starts; the rest are offline and may
    /// be started, or started and given up on, at any point during it.
    pub online_at_start: usize,
    pub flush: Flush,
    /// `select` targets a CPU whose descriptor reads `Starting` as well as
    /// `Online`. The boot CPU writes `Starting` before it sends INIT, so a
    /// CPU `select` reads as `Offline` has an empty TLB from after the write.
    /// The mutant targets `Online` alone, as the code did.
    pub target_starting: bool,
    /// The waiter acknowledges for a target the boot CPU gave up on and set
    /// back to `Offline`: INIT emptied its TLB, and it will never answer.
    pub ack_for_abandoned: bool,
    /// The boot CPU may ask an AP that entered to park, at any point of its
    /// bring-up. Off, the search covers the round without that path, which
    /// keeps machines of four and five CPUs within memory.
    pub may_ask_park: bool,
    /// The steps of `bring_up`, where an AP the boot CPU asked to park may
    /// park.
    pub bring_up: &'static [Step],
    /// An AP the boot CPU asked to park leaves the target set only once it
    /// has parked and INIT has reset it. The mutant drops it at the request,
    /// while it may still be running with what it cached.
    pub drop_after_init: bool,
    /// The initiator writes the page table before it selects targets.
    pub write_before_select: bool,
    /// A target flushes before it acknowledges.
    pub flush_before_ack: bool,
    /// The initiator's CR4.PGE. Kernel PTEs are GLOBAL, so with PGE set a CR3
    /// reload leaves the page cached on that CPU.
    pub initiator_pge: bool,
}

impl Config {
    pub const fn kernel(cpus: usize, online_at_start: usize, flush: Flush) -> Self {
        Config {
            cpus,
            online_at_start,
            flush,
            target_starting: true,
            ack_for_abandoned: true,
            may_ask_park: true,
            bring_up: BRING_UP,
            drop_after_init: true,
            write_before_select: true,
            flush_before_ack: true,
            initiator_pge: false,
        }
    }
}

pub const MAX_CPUS: usize = 5;

/// Where an AP is. The descriptor reads `Offline` for `Offline` and
/// `Abandoned`, `Starting` for `Starting`, `Woken`, `Entered` and `Parked`,
/// and `Online` for `Online`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Ap {
    Offline,
    /// `configure_descriptor` stored `Starting`; INIT not yet delivered.
    Starting,
    /// INIT and STARTUP delivered: running the trampoline on the kernel's
    /// CR3 with an empty TLB, before its claim.
    Woken,
    /// Claimed its start (`ap_entry`); at step `n` of `bring_up`, interrupts
    /// masked. At its spin points (`round`'s lock loop, `lock_responsive`) it
    /// serves its own pending flag.
    Entered(u8),
    /// Asked to park, it parked itself at a checkpoint: `cli; hlt`, its
    /// claim word `Parked`. Still `Starting` to everyone else until the boot
    /// CPU has seen that and INIT has reset it.
    Parked,
    /// `publish` stored `Online`; interrupts on.
    Online,
    /// The boot CPU took the claim, stored `Offline` and sent INIT: halted,
    /// TLB empty, never running kernel code again.
    Abandoned,
}

/// The target side of one round, from the pending flag to the ack.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Handler {
    Idle,
    /// Took its pending flag (`swap(false)`).
    Taken,
    /// Has done one of flush and ack, and owes the other.
    Half,
}

/// Where the initiator is in `flush_tlb_*_smp` and `broadcast`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Init {
    Start,
    Written,
    LocalFlushed,
    /// Reading CPU `n`'s descriptor for `select`.
    Selecting(u8),
    /// Setting the pending flag of the `n`th CPU and sending to it.
    Sending(u8),
    Waiting,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct State {
    pub pte: Val,
    pub init: Init,
    pub targets: u8,
    pub acks: u8,
    pub tlb: [Option<Val>; MAX_CPUS],
    pub ap: [Ap; MAX_CPUS],
    pub pending: [bool; MAX_CPUS],
    pub handler: [Handler; MAX_CPUS],
    /// The boot CPU stored `AbandonRequested` in this AP's claim word: it
    /// missed its online deadline.
    pub park_asked: [bool; MAX_CPUS],
}

impl State {
    pub fn start(cfg: &Config) -> State {
        let mut s = State {
            pte: Val::Old,
            init: Init::Start,
            targets: 0,
            acks: 0,
            tlb: [None; MAX_CPUS],
            ap: [Ap::Offline; MAX_CPUS],
            pending: [false; MAX_CPUS],
            handler: [Handler::Idle; MAX_CPUS],
            park_asked: [false; MAX_CPUS],
        };
        // The initiator, and every AP online at the start, may hold the old
        // translation already.
        s.ap[0] = Ap::Online;
        s.tlb[0] = Some(Val::Old);
        for c in 1..=cfg.online_at_start {
            s.ap[c] = Ap::Online;
            s.tlb[c] = Some(Val::Old);
        }
        s
    }

    /// Runs kernel code on the kernel's page tables, so may fill its TLB.
    pub fn runs_kernel(&self, c: usize) -> bool {
        matches!(self.ap[c], Ap::Woken | Ap::Entered(_) | Ap::Online)
    }

    /// Takes the shootdown IPI, or the NMI the waiter sends at 50 ms.
    fn interruptible(&self, c: usize) -> bool {
        self.ap[c] == Ap::Online
    }

    /// Serves its own pending flag, by interrupt or at a spin point.
    fn serves(&self, c: usize) -> bool {
        matches!(self.ap[c], Ap::Entered(_) | Ap::Online)
    }

    fn descriptor_offline(&self, c: usize, cfg: &Config) -> bool {
        matches!(self.ap[c], Ap::Offline | Ap::Abandoned)
            || (!cfg.drop_after_init && self.park_asked[c] && self.ap[c] != Ap::Online)
    }

    fn descriptor_starting(&self, c: usize, cfg: &Config) -> bool {
        matches!(self.ap[c], Ap::Starting | Ap::Woken | Ap::Entered(_) | Ap::Parked)
            && !self.descriptor_offline(c, cfg)
    }

    /// Holds a lock taken in `bring_up`: an odd number of steps since the
    /// last release.
    fn holds_lock(&self, c: usize, cfg: &Config) -> bool {
        let Ap::Entered(pc) = self.ap[c] else { return false };
        cfg.bring_up[..pc as usize].iter().fold(false, |held, step| match step {
            Step::Lock => true,
            Step::Unlock => false,
            Step::Checkpoint => held,
        })
    }
}

/// A CPU drops the page from its TLB. `full` is a CR3 reload, which keeps a
/// GLOBAL entry when that CPU has CR4.PGE set. APs never set PGE.
fn drop_page(s: &mut State, cfg: &Config, c: usize, full: bool) {
    let pge = c == 0 && cfg.initiator_pge;
    if !(full && pge) {
        s.tlb[c] = None;
    }
}

/// Every state one step away from `s`.
pub fn successors(s: &State, cfg: &Config, out: &mut Vec<State>) {
    out.clear();
    let n = cfg.cpus;

    // The hardware: any CPU running kernel code may fill or evict.
    for c in 0..n {
        if s.runs_kernel(c) {
            let mut t = *s;
            t.tlb[c] = match s.tlb[c] {
                None => Some(s.pte),
                Some(_) => None,
            };
            out.push(t);
        }
    }

    // AP bring-up, and the boot CPU giving up on an AP that never claimed.
    for c in 1..n {
        let mut t = *s;
        match s.ap[c] {
            Ap::Offline => t.ap[c] = Ap::Starting,
            Ap::Starting => {
                // INIT empties the TLB.
                t.tlb[c] = None;
                t.ap[c] = Ap::Woken;
            }
            Ap::Woken => t.ap[c] = Ap::Entered(0),
            Ap::Entered(pc) => {
                let pc = pc as usize;
                // With no park request possible the steps of `bring_up` are
                // all alike to the round, so one step stands for them.
                if pc == cfg.bring_up.len() || !cfg.may_ask_park {
                    // `go_live`: interrupts on, `Online` published.
                    t.ap[c] = Ap::Online;
                } else if cfg.bring_up[pc] == Step::Checkpoint && s.park_asked[c] {
                    // CAS to `Parked`, then `cli; hlt`.
                    t.ap[c] = Ap::Parked;
                } else {
                    t.ap[c] = Ap::Entered(pc as u8 + 1);
                }
            }
            Ap::Parked => {
                // The boot CPU saw `Parked` and sent INIT, which succeeded:
                // the core is reset, its TLB empty, and it is `Offline`.
                t.ap[c] = Ap::Abandoned;
                t.tlb[c] = None;
            }
            Ap::Online | Ap::Abandoned => continue,
        }
        out.push(t);
        // Past its online deadline the boot CPU asks an AP that entered to
        // park; it may do so at any point of the AP's bring-up.
        if cfg.may_ask_park && matches!(s.ap[c], Ap::Entered(_)) && !s.park_asked[c] {
            let mut t = *s;
            t.park_asked[c] = true;
            out.push(t);
        }
        if matches!(s.ap[c], Ap::Starting | Ap::Woken) {
            let mut t = *s;
            t.ap[c] = Ap::Abandoned;
            t.tlb[c] = None;
            out.push(t);
        }
    }

    // Targets.
    for c in 1..n {
        let mut t = *s;
        let full = cfg.flush == Flush::Full;
        match s.handler[c] {
            Handler::Idle if s.pending[c] && (s.interruptible(c) || s.serves(c)) => {
                t.pending[c] = false;
                t.handler[c] = Handler::Taken;
            }
            Handler::Taken => {
                if cfg.flush_before_ack {
                    drop_page(&mut t, cfg, c, full);
                } else {
                    t.acks -= 1;
                }
                t.handler[c] = Handler::Half;
            }
            Handler::Half => {
                if cfg.flush_before_ack {
                    t.acks -= 1;
                } else {
                    drop_page(&mut t, cfg, c, full);
                }
                t.handler[c] = Handler::Idle;
            }
            _ => continue,
        }
        out.push(t);
    }

    // The initiator.
    let mut t = *s;
    let full = cfg.flush == Flush::Full;
    match s.init {
        Init::Start => {
            if cfg.write_before_select {
                t.pte = Val::New;
                t.init = Init::Written;
            } else {
                t.init = Init::Selecting(1);
            }
        }
        Init::Written => {
            drop_page(&mut t, cfg, 0, full);
            t.init = if cfg.write_before_select { Init::LocalFlushed } else { Init::Sending(1) };
        }
        Init::LocalFlushed => t.init = Init::Selecting(1),
        Init::Selecting(i) => {
            let c = i as usize;
            if c < n {
                let target =
                    s.ap[c] == Ap::Online || (cfg.target_starting && s.descriptor_starting(c, cfg));
                if target {
                    t.targets |= 1 << c;
                }
                t.init = Init::Selecting(i + 1);
            } else if cfg.write_before_select {
                t.init = Init::Sending(1);
            } else {
                // The mutant: targets chosen, the write only now.
                t.pte = Val::New;
                t.init = Init::Written;
            }
        }
        Init::Sending(i) => {
            let c = i as usize;
            if c < n {
                if s.targets & (1 << c) != 0 {
                    t.acks += 1;
                    t.pending[c] = true;
                }
                t.init = Init::Sending(i + 1);
            } else {
                t.init = Init::Waiting;
            }
        }
        Init::Waiting => {
            if s.acks == 0 {
                t.init = Init::Done;
                out.push(t);
                return;
            }
            // The waiter's own pass over targets the boot CPU gave up on: it
            // takes the pending flag, so exactly one side acknowledges.
            if cfg.ack_for_abandoned {
                for c in 1..n {
                    if s.pending[c] && s.descriptor_offline(c, cfg) {
                        let mut t = *s;
                        t.pending[c] = false;
                        t.acks -= 1;
                        out.push(t);
                    }
                }
            }
            return;
        }
        Init::Done => return,
    }
    out.push(t);
}

/// What a correct round guarantees, checked in every reachable state.
pub fn check(s: &State, cfg: &Config) -> Result<(), &'static str> {
    for c in 1..cfg.cpus {
        // An AP that parks inside a lock leaves it held for ever, and every
        // later taker spins: the wedge parking exists to avoid. It parks at a
        // checkpoint once asked, so a checkpoint reached while holding a lock
        // with the request set is that wedge.
        if let Ap::Entered(pc) = s.ap[c] {
            let pc = pc as usize;
            if s.park_asked[c]
                && pc < cfg.bring_up.len()
                && cfg.bring_up[pc] == Step::Checkpoint
                && s.holds_lock(c, cfg)
            {
                return Err("an AP can park while holding a lock");
            }
        }
    }
    // Each outstanding ack belongs to a target that will answer it: one that
    // serves its flag now or will once it comes online, or one the waiter
    // answers for. Without that the wait reaches its deadline and stops the
    // machine.
    if matches!(s.init, Init::Waiting) {
        let mut owed = 0u8;
        for c in 1..cfg.cpus {
            let open = s.pending[c] || s.handler[c] != Handler::Idle;
            if !open {
                continue;
            }
            if s.targets & (1 << c) == 0 {
                return Err("a cpu that was not targeted owes an ack");
            }
            if s.ap[c] == Ap::Abandoned && !cfg.ack_for_abandoned {
                return Err("an ack is owed by a cpu that will never answer");
            }
            if s.handler[c] != Handler::Half || cfg.flush_before_ack {
                owed += 1;
            }
        }
        if owed != s.acks {
            return Err("the ack count does not match the open targets");
        }
    }
    // Once the round returns, no CPU that can still run kernel code holds the
    // old translation, and neither can one that has yet to start.
    if s.init == Init::Done {
        for c in 0..cfg.cpus {
            if s.tlb[c] == Some(Val::Old) && s.ap[c] != Ap::Abandoned {
                return Err("a cpu holds the old translation after the round returned");
            }
        }
    }
    Ok(())
}
