# login_proofs

Host-runnable proofs for the login capsule's session, on the real source
(`capsule_login/src/state`, included through `#[path]`).

- One session at a time: a second start is refused as busy, and only the
  session's owner can end it.
- `end_if_owner_ended` locks the session of an owner that ended without
  ending it, gives back the key it was opened with, and a new session can
  start. A living owner keeps its session, and a locked machine stays locked.

What is not proven here: that the serve loop asks before each request
(`server/reap.rs`), that the shell is told and the lock screen redrawn, and
that `mk_pid_alive` answers truly. Those need a boot.

Run: `cargo test` in this directory.

The login capsule is described in
[System apps and services](../../docs/handbook/apps/system-apps.md).
