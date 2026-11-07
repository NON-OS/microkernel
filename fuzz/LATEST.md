# Latest fuzz run

`eb0f76de3b1c`, 2026-11-07T08:55:25+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/42354219337

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3388910 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1273424096 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 198009173 | 303 | +0 | 437 | +0 | 347 | +0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 209329556 | 339 | +0 | 618 | +0 | 445 | +0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 3196151 | 1300 | +2 | 4331 | +12 | 5685 | +294 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 87509388 | 479 | +0 | 2558 | +0 | 3383 | +134 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1548794969 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1273618062 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 336662258 | 107 | +0 | 110 | +3 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 223286363 | 217 | +0 | 841 | +1 | 1506 | +74 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 774420615 | 212 | +0 | 497 | +1 | 1857 | +26 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 3038808 | 2065 | +0 | 4638 | +4 | 682 | +4 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1038874963 | 111 | +0 | 314 | +5 | 483 | +18 | 0 |
