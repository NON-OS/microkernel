// NONOS Operating System (AGPL-3.0-or-later)
//! A command the shell hands the Terminal from a Launchpad tool tile. Only a
//! `run:` or `type:` reply is a command, a path never is, and nothing the
//! keyboard could not have typed is put on the prompt. The command lands in
//! the untouched tab the tile just opened, or a tab of its own, and never
//! over a program or a half-typed line.

mod cadence;
mod pick;
mod prefixes;
mod refusals;

/// A reply body as the shell sends it: the 4-byte status, then the argument.
fn reply(arg: &[u8]) -> Vec<u8> {
    let mut body = vec![0u8; 4];
    body.extend_from_slice(arg);
    body
}
