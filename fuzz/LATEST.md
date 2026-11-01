# Latest fuzz run

`eb0f76de3b1c`, 2026-11-01T09:19:42+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/41289418446

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 3869110 | 241 |  | 432 |  | 39 |  | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1241036091 | 81 |  | 83 |  | 85 |  | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 231090121 | 303 |  | 437 |  | 340 |  | 0 |
| `userland/admission_proofs` | `manifest` | pass | 301865973 | 339 |  | 618 |  | 441 |  | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 2473500 | 1274 |  | 4157 |  | 2140 |  | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 65850127 | 479 |  | 2556 |  | 1785 |  | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1480075114 | 177 |  | 178 |  | 43 |  | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1321506217 | 35 |  | 36 |  | 6 |  | 0 |
| `userland/market_proto` | `linux_policy` | pass | 305164758 | 107 |  | 107 |  | 12 |  | 0 |
| `userland/net_proofs` | `dns` | pass | 220021080 | 217 |  | 821 |  | 792 |  | 0 |
| `userland/net_proofs` | `net_packets` | pass | 524546041 | 205 |  | 459 |  | 1305 |  | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 3692722 | 2061 |  | 4620 |  | 662 |  | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 922412499 | 109 |  | 294 |  | 181 |  | 0 |
