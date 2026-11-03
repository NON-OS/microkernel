# Latest fuzz run

`eb0f76de3b1c`, 2026-11-03T09:49:18+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/41606855436

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 4405455 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1220233471 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 190176828 | 303 | +0 | 437 | +0 | 341 | +1 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 6854906 | 1286 | +8 | 4257 | +62 | 3704 | +865 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 67941572 | 479 | +0 | 2556 | +0 | 2734 | +286 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 1142053374 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1266588912 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 309308028 | 107 | +0 | 107 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 317350673 | 217 | +0 | 821 | +0 | 1246 | +151 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 577546466 | 210 | +2 | 479 | +3 | 1718 | +136 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 3400133 | 2061 | +0 | 4627 | +5 | 667 | +4 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 1308651423 | 111 | +1 | 303 | +6 | 364 | +75 | 0 |
