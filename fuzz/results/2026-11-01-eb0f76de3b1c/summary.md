## Fuzz run `eb0f76de3b1c` (schedule)

13 targets, 13 passed, 0 crash inputs. https://github.com/NON-OS/microkernel/actions/runs/41289418446

| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 1800 | 3869110 | 2148 | 241 | 432 | 39 | 0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1800 | 1241036091 | 689081 | 81 | 83 | 85 | 0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 1800 | 231090121 | 128312 | 303 | 437 | 340 | 0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 1800 | 301865973 | 167610 | 339 | 618 | 441 | 3 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 1800 | 2473500 | 1373 | 1274 | 4157 | 2140 | 697 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 1800 | 65850127 | 36563 | 479 | 2556 | 1785 | 1237 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1800 | 1480075114 | 821807 | 177 | 178 | 43 | 0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1800 | 1321506217 | 733762 | 35 | 36 | 6 | 0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 1800 | 305164758 | 169441 | 107 | 107 | 12 | 0 | 0 |
| `userland/net_proofs` | `dns` | pass | 1800 | 220021080 | 122166 | 217 | 821 | 792 | 354 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 1800 | 524546041 | 291252 | 205 | 459 | 1305 | 721 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 1800 | 3692722 | 2050 | 2061 | 4620 | 662 | 4 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1800 | 922412499 | 512166 | 109 | 294 | 181 | 66 | 0 |
