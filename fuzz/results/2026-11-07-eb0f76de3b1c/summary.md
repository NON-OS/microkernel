## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/42354219337

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3388910 | 1881 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1273424096 | 707065 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 198009173 | 109944 | 303 | 437 | 347 | 0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 209329556 | 116229 | 339 | 618 | 445 | 0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 3196151 | 1774 | 1300 | 4331 | 5685 | 294 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 87509388 | 48589 | 479 | 2558 | 3383 | 134 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1548794969 | 859963 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1273618062 | 707172 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 336662258 | 186930 | 107 | 110 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 223286363 | 123979 | 217 | 841 | 1506 | 74 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 774420615 | 429994 | 212 | 497 | 1857 | 26 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 3038808 | 1687 | 2065 | 4638 | 682 | 4 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 1038874963 | 576832 | 111 | 314 | 483 | 18 | 0 |
