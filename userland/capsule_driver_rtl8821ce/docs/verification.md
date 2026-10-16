## Verification

- Capsule: `(cd userland/capsule_driver_rtl8821ce && cargo build --release
 --target../x86_64-nonos-user.json -Zbuild-std=core,alloc
 -Zbuild-std-features=compiler-builtins-mem)`
- Proofs: `(cd userland/rtl8821ce_proofs && cargo test --release)` and
 `(cd userland/nonos_wifi_core_proofs && cargo test --release)`
- Build: `make -B nonos-mk-driver-rtl8821ce`
- Kernel profile: `cargo check --no-default-features --features
 microkernel-driver-rtl8821ce`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Handbook: [drivers](../../../docs/handbook/drivers.md),
 [network stack](../../../docs/handbook/network/stack.md).
