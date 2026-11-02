## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41415182648

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3445215 | 1912 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1244479356 | 690993 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 179895366 | 99886 | 303 | 437 | 340 | 0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 256068119 | 142181 | 339 | 618 | 443 | 2 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 4492665 | 2494 | 1278 | 4195 | 2839 | 699 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 112224251 | 62312 | 479 | 2556 | 2448 | 663 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1475083089 | 819035 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1285970327 | 714031 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 305562483 | 169662 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 329486533 | 182946 | 217 | 821 | 1095 | 303 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 827824587 | 459647 | 208 | 476 | 1582 | 277 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 4422631 | 2455 | 2061 | 4622 | 663 | 1 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 891519601 | 495013 | 110 | 297 | 289 | 108 | 0 |
