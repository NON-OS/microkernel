# Latest fuzz run

`eb0f76de3b1c`, 2026-11-06T09:46:25+00:00, schedule. https://github.com/NON-OS/microkernel/actions/runs/42190615443

Coverage and corpus against the previous run of each target for the same length of time.

| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `nonos-attest-path` | `v3_path` | pass | 5253190 | 241 | +0 | 432 | +0 | 39 | +0 | 0 |
| `nonos-attest-path` | `v4_parse` | pass | 1539948691 | 81 | +0 | 83 | +0 | 85 | +0 | 0 |
| `userland/admission_proofs` | `id_cert` | pass | 213341538 | 303 | +0 | 437 | +0 | 347 | +0 | 0 |
| `userland/admission_proofs` | `manifest` | pass | 216861381 | 339 | +0 | 618 | +0 | 445 | +0 | 0 |
| `userland/capsule_linux_proofs` | `linux_inputs` | pass | 6361456 | 1298 | +3 | 4319 | +18 | 5391 | +513 | 0 |
| `userland/capsule_linux_proofs` | `serve_wire` | pass | 76580247 | 479 | +0 | 2558 | +1 | 3249 | +156 | 0 |
| `userland/driver_proofs` | `ipc_decoders` | pass | 2013555729 | 177 | +0 | 178 | +0 | 43 | +0 | 0 |
| `userland/kernel_proofs` | `elf_header` | pass | 1271254757 | 35 | +0 | 36 | +0 | 6 | +0 | 0 |
| `userland/market_proto` | `linux_policy` | pass | 371409963 | 107 | +0 | 107 | +0 | 12 | +0 | 0 |
| `userland/net_proofs` | `dns` | pass | 224947649 | 217 | +0 | 840 | +4 | 1432 | +62 | 0 |
| `userland/net_proofs` | `net_packets` | pass | 881612809 | 212 | +0 | 496 | +2 | 1831 | +27 | 0 |
| `userland/tls_proofs` | `tls_cert` | pass | 2744849 | 2065 | +0 | 4634 | +1 | 678 | +2 | 0 |
| `userland/tls_proofs` | `tls_record` | pass | 821591351 | 111 | +0 | 309 | +3 | 465 | +15 | 0 |
