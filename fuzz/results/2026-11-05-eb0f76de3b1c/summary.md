## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41997746574

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3270040 | 1815 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1252894481 | 695666 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 292234653 | 162262 | 303 | 437 | 347 | 4 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 203657869 | 113080 | 339 | 618 | 445 | 1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 6685380 | 3712 | 1295 | 4301 | 4878 | 599 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 71676381 | 39798 | 479 | 2557 | 3093 | 168 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1997403539 | 1109052 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1857103052 | 1031151 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 301730623 | 167535 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 175387983 | 97383 | 217 | 836 | 1370 | 65 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 800278042 | 444352 | 212 | 494 | 1804 | 26 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 4006076 | 2224 | 2065 | 4633 | 676 | 6 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 1156326328 | 642046 | 111 | 306 | 450 | 36 | 0 |
