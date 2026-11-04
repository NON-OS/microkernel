## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41799666571

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3250195 | 1804 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1234991746 | 685725 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 166131635 | 92244 | 303 | 437 | 343 | 2 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 213623851 | 118614 | 339 | 618 | 444 | 1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 4654801 | 2584 | 1294 | 4281 | 4279 | 575 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 62319472 | 34602 | 479 | 2556 | 2925 | 191 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1142028864 | 634108 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1273864517 | 707309 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 289216818 | 160586 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 255306041 | 141757 | 217 | 833 | 1305 | 59 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 820653893 | 455665 | 210 | 487 | 1778 | 60 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 3072124 | 1705 | 2064 | 4630 | 670 | 3 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 764885099 | 424700 | 111 | 303 | 414 | 50 | 0 |
