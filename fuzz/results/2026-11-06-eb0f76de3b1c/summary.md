## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/42190615443

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 5253190 | 2916 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1539948691 | 855052 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 213341538 | 118457 | 303 | 437 | 347 | 0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 216861381 | 120411 | 339 | 618 | 445 | 0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 6361456 | 3532 | 1298 | 4319 | 5391 | 513 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 76580247 | 42520 | 479 | 2558 | 3249 | 156 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 2013555729 | 1118020 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1271254757 | 705860 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 371409963 | 206224 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 224947649 | 124901 | 217 | 840 | 1432 | 62 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 881612809 | 489512 | 212 | 496 | 1831 | 27 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 2744849 | 1524 | 2065 | 4634 | 678 | 2 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 821591351 | 456186 | 111 | 309 | 465 | 15 | 0 |
