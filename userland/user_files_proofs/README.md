# user_files_proofs

`nonos_user_files` on the host, over doubles for the volume and the kernel
(`src/doubles.rs`).

- `reference_tests`: thousands of random calls made of the broker and of
  Linux on a temporary directory, every answer compared (errno, bytes, sizes,
  listings), with a reboot every 400 calls.
- `crash_tests`: a safe save (temporary name, write, fsync, rename) cut at
  every write it makes, torn writes too; the next start finds the old
  document or the new one and no object nothing needs.
- `reboot_tests`, `two_client_tests`, `size_tests`, `path_tests`,
  `wire_tests`: persistence with RAM lost, two programs at once, large files
  in bounded memory, quota and full volume, path escapes, every op's errno.
- `fuzz_tests` and the cargo-fuzz target `user_files_wire` (`fuzz/`): any
  bytes as requests, as a manifest body and as a slot.

```sh
cargo +nightly-2026-01-16 test --release
cd fuzz && cargo +nightly-2026-01-16 fuzz run user_files_wire
```
