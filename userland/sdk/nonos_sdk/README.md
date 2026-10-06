# nonos-sdk

`sdk_main!(app, caps: [WINDOW, NETWORK])` writes the app's capability word into a
`NONOS_DECLARED_CAPS` static in a `.nonos.caps` section and emits `_start`,
which calls `nonos_runtime::run` with that word. The word is `BASE` (CoreExec
and Memory) ORed with the named groups: `WINDOW` (the four graphics bits),
`NETWORK`, `STORAGE` (FileSystem), `CRYPTO`, `IPC`, `SERVICE` (RegisterService
and IPC), `DEBUG` and `BUILD_TOOLING` (EnrolDevRoot).

The section is not what the kernel enforces. The kernel installs the word from
the signed manifest, whose number comes from the capsule's `Capsule.mk`.
`scripts/check_declared_caps.py` compares the two after signing, in the seal's
verify phase and in `nonos-mk-verify-image`. See [the libc and std page](../../../docs/handbook/userland/libc-and-std.md).
