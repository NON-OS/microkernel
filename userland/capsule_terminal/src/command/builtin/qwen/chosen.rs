/*
 * The tier `qwen` runs when none is named: the one chosen at setup or in
 * Settings when it is a tier this terminal knows, else the first.
 *
 * Asked of the policy service on every `qwen` command, never cached: a
 * tier chosen in Settings while this Terminal is open is the one the next
 * `qwen` runs. One short IPC call per command, at most 250 ms when the
 * policy service does not answer; never per keystroke or per frame.
 *
 * When nothing was chosen, the answer could not be had, or it names no tier
 * this terminal runs, the default runs (`default_tier`: the stick tier when
 * it fits, else the largest that fits) and the command says so
 * (`take_note`): a tier the person did not choose never runs in silence.
 */

use core::sync::atomic::{AtomicU8, Ordering};

use alloc::vec::Vec;

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use super::default_tier::{resolve, Source, WEIGHTS};
use super::need::Room;
use super::tiers::{default_line, pick_said, UNANSWERED, UNKNOWN};
use crate::term::identity::policy_string;

/* The memory the kernel counts, read as `qwen tiers` reads it. */
#[path = "../../../../../capsule_model_fetch/src/tiers/memory.rs"]
mod memory;

/* What the last `chosen` has to say: 0 nothing, 1 UNANSWERED, 2 UNKNOWN. */
static NOTE: AtomicU8 = AtomicU8::new(0);
/* The default the last `chosen` ran, as an index into WEIGHTS plus one; 0 for none. */
static DEFAULT: AtomicU8 = AtomicU8::new(0);
static SOURCE: AtomicU8 = AtomicU8::new(0);

pub fn chosen() -> &'static [u8] {
    let mut reply = [0u8; 64];
    let got = policy_string(Field::QwenTier, &mut reply);
    let (tier, note) = pick_said(got.map(|n| &reply[..n]));
    let code = match note {
        Some(n) if n == UNANSWERED => 1,
        Some(_) => 2,
        None => 0,
    };
    NOTE.store(code, Ordering::Relaxed);
    if let Some(tier) = tier {
        DEFAULT.store(0, Ordering::Relaxed);
        return tier;
    }
    let room = match lookup().and_then(|port| get_bool(port, Field::Persistent)) {
        Some(true) => Room::Disk,
        _ => Room::Memory,
    };
    let (tier, source) = resolve(b"", memory::memory(), room);
    let at = WEIGHTS.iter().position(|w| w.0 == tier).map_or(0, |i| i + 1);
    DEFAULT.store(at as u8, Ordering::Relaxed);
    SOURCE.store(source as u8, Ordering::Relaxed);
    tier.as_bytes()
}

/*
 * The lines the last `chosen` left, once: why the choice could not be read,
 * if it could not, and which default runs and why.
 */
pub fn take_note() -> Option<Vec<u8>> {
    let mut out = match NOTE.swap(0, Ordering::Relaxed) {
        1 => UNANSWERED.to_vec(),
        2 => UNKNOWN.to_vec(),
        _ => Vec::new(),
    };
    let at = usize::from(DEFAULT.swap(0, Ordering::Relaxed));
    let source = match SOURCE.load(Ordering::Relaxed) {
        1 => Source::Stick,
        2 => Source::Largest,
        _ => Source::Smallest,
    };
    if let Some(line) = at.checked_sub(1).and_then(|i| default_line(WEIGHTS[i].0, source)) {
        if !out.is_empty() {
            out.push(b'\n');
        }
        out.extend_from_slice(line.as_bytes());
    }
    (!out.is_empty()).then_some(out)
}
