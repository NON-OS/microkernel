# capsule_std_proof

An unmodified-std Rust program that shows the NONOS std platform layer works
inside the syscall boundary. It is built with `CAPSULE_BUILD_STD :=
std,panic_abort`, uses `serde_json` and `base64` on a JSON document, does file
I/O through `std::fs` and checks threads. The std layer is described in
[docs/handbook/userland/libc-and-std.md](../../docs/handbook/userland/libc-and-std.md).

- `CAPSULE_REQUIRED_CAPS = 0x59`: CoreExec, IPC, Memory and FileSystem.
  FileSystem is for the file I/O proof, whose `std::fs` calls go to vfs, and
  vfs serves only a holder of FileSystem. Debug (`0x100`) is optional: the
  output reaches the `proc.<pid>` inbox the terminal drains either way, and a
  `capsule-serial-debug` build adds the copy on the serial line. It requests
  no hardware, raw storage or network authority.
- Service `service:4502:std_proof`, reply `reply:4503:endpoint.std_proof.reply`.
- No profile turns on `nonos-capsule-std-proof`. The capsule reaches a running
  system through the store disk (`NONOS_STORE_DEMO_ENTRIES` in
  `mk/40-run.mk`), and the terminal runs it by its bare name `std_proof`
  through the installer.
