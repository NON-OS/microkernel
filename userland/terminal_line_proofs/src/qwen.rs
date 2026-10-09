// NONOS Operating System (AGPL-3.0-or-later)
//! `qwen`'s pure parts, compiled from the terminal's own sources: the line
//! as typed, the tier allowlist, the window and fetch requests, and what Tab
//! offers.

#[path = "../../capsule_terminal/src/command/builtin/qwen/ask.rs"]
pub mod ask;
#[path = "../../capsule_terminal/src/command/builtin/qwen/complete.rs"]
pub mod complete;
#[path = "../../capsule_terminal/src/command/builtin/qwen/fetch_check.rs"]
pub mod fetch_check;
#[path = "../../capsule_terminal/src/command/builtin/qwen/fetch_words.rs"]
pub mod fetch_words;
#[path = "../../capsule_terminal/src/command/builtin/qwen/tiers.rs"]
pub mod tiers;
#[path = "../../capsule_terminal/src/command/builtin/qwen/window.rs"]
pub mod window;

/*
 * The host has no policy server: `chosen` is the default an unset policy
 * runs on a machine whose memory is not known, which is the stick tier.
 */
pub mod chosen {
    pub fn chosen() -> &'static [u8] {
        super::default_tier::resolve(b"", None, super::need::Room::Memory).0.as_bytes()
    }
}

/* Which tier runs when none is named, and the fit rule it weighs by. */
#[path = "../../capsule_model_fetch/src/default_tier.rs"]
pub mod default_tier;
#[path = "../../capsule_model_fetch/src/need.rs"]
pub mod need;

#[path = "../../capsule_terminal/src/term/util/is_space.rs"]
pub mod space;

/*
 * The kernel's allowlist, the one a request is checked against before
 * anything is spawned, so the tests hold the terminal's words to it.
 */
#[cfg(test)]
#[path = "../../../src/userspace/capsule_linux/terminal/tier.rs"]
mod kernel_tier;

#[cfg(test)]
#[path = "qwen_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "qwen_fetch_tests.rs"]
mod fetch_tests;

#[cfg(test)]
#[path = "qwen_pick_tests.rs"]
mod pick_tests;
