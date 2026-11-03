## Fuzz run `eb0f76de3b1c` (schedule)

12 targets, 12 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41606855436

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 4405455 | 2446 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1220233471 | 677531 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 190176828 | 105595 | 303 | 437 | 341 | 1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 6854906 | 3806 | 1286 | 4257 | 3704 | 865 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 67941572 | 37724 | 479 | 2556 | 2734 | 286 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1142053374 | 634121 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1266588912 | 703269 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 309308028 | 171742 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 317350673 | 176208 | 217 | 821 | 1246 | 151 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 577546466 | 320680 | 210 | 479 | 1718 | 136 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 3400133 | 1887 | 2061 | 4627 | 667 | 4 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 1308651423 | 726624 | 111 | 303 | 364 | 75 | 0 |
