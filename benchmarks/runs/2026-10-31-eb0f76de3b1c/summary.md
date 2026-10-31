# NØNOS Benchmark Run

Output: `benchmarks/runs/41225526739-1`

- `boot-evidence` status `pass` elapsed `448424` ms
- `nonos.bench.bootlog.v1` status `pass` elapsed `` ms
- `nonos.bench.boot.matrix.v1` status `pass` elapsed `` ms
- `build-verify-fast` status `pass` elapsed `194600` ms
- `nonos.bench.host.v1` status `recorded` elapsed `` ms

Boot phases:
- `capsule_attest_ms` `83` ms
- `capsule_attest_to_runqueue_ms` `10` ms
- `capsule_install_ms` `6` ms
- `capsule_preflight_ms` `87` ms
- `first_display_flush_ms` `3600` ms
- `first_spawn_ms` `93` ms
- `first_surface_register_ms` `3426` ms
- `kernel_core_ms` `123` ms
- `microkernel_init_ms` `130` ms
- `userspace_entry_ms` `2496` ms

Sample aggregates:
- `capsule_attest_ms` count `28` min `66` avg `85.179` max `389` ms
- `capsule_attest_to_runqueue_ms` count `28` min `3` avg `6.0` max `27` ms
- `capsule_install_ms` count `28` min `1` avg `3.75` max `25` ms
- `capsule_preflight_ms` count `28` min `68` avg `87.429` max `391` ms
- `first_display_flush_ms` count `1` min `3600` avg `3600.0` max `3600` ms
- `first_spawn_ms` count `28` min `69` avg `91.179` max `416` ms
- `first_surface_register_ms` count `1` min `3426` avg `3426.0` max `3426` ms
- `kernel_core_ms` count `1` min `123` avg `123.0` max `123` ms
- `microkernel_init_ms` count `1` min `130` avg `130.0` max `130` ms
- `userspace_entry_ms` count `1` min `2496` avg `2496.0` max `2496` ms

Boot matrix aggregates:
- `capsule_attest_ms` count `1` min `83` avg `83.0` max `83` ms
- `capsule_attest_to_runqueue_ms` count `1` min `10` avg `10.0` max `10` ms
- `capsule_install_ms` count `1` min `6` avg `6.0` max `6` ms
- `capsule_preflight_ms` count `1` min `87` avg `87.0` max `87` ms
- `first_display_flush_ms` count `1` min `3600` avg `3600.0` max `3600` ms
- `first_spawn_ms` count `1` min `93` avg `93.0` max `93` ms
- `first_surface_register_ms` count `1` min `3426` avg `3426.0` max `3426` ms
- `kernel_core_ms` count `1` min `123` avg `123.0` max `123` ms
- `microkernel_init_ms` count `1` min `130` avg `130.0` max `130` ms
- `userspace_entry_ms` count `1` min `2496` avg `2496.0` max `2496` ms
