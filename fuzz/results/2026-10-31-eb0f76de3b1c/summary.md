## Fuzz run `eb0f76de3b1c` (push)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41225526739

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 120 | 291358 | 2407 | 241 | 432 | 39 | 1 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 120 | 83343781 | 688791 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 120 | 17314461 | 143094 | 303 | 437 | 340 | 0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 120 | 23532584 | 194484 | 339 | 618 | 438 | 0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 120 | 503392 | 4160 | 1265 | 4125 | 1443 | 205 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 120 | 3008745 | 24865 | 479 | 2548 | 548 | 222 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 120 | 130960869 | 1082321 | 177 | 178 | 43 | 1 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 120 | 95111904 | 786048 | 35 | 36 | 6 | 1 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 120 | 22381298 | 184969 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 120 | 16756583 | 138484 | 213 | 805 | 438 | 67 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 120 | 52717076 | 435678 | 204 | 457 | 584 | 380 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 120 | 388769 | 3212 | 2060 | 4613 | 658 | 1 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 120 | 83680194 | 691571 | 104 | 228 | 115 | 31 | 0 |
