## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/42463004789

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3389155 | 1881 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 2080574591 | 1155232 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 198798319 | 110382 | 303 | 437 | 348 | 1 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 239367921 | 132908 | 339 | 618 | 445 | 0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 5196548 | 2885 | 1300 | 4350 | 6108 | 423 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 91483148 | 50795 | 479 | 2558 | 3474 | 91 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1168068094 | 648566 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1294078227 | 718533 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 343654518 | 190813 | 107 | 110 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 257889501 | 143192 | 217 | 844 | 1603 | 97 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 863470221 | 479439 | 212 | 499 | 1869 | 12 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 3311090 | 1838 | 2065 | 4641 | 685 | 3 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 1172591858 | 651078 | 111 | 314 | 510 | 27 | 0 |
