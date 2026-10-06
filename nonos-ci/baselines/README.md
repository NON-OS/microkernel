# CI baselines

Each file holds the current count for one source-tree property. The
`static-checks` job in `microkernel-baseline.yml` fails when the actual
count exceeds the file.

| File | Counts | Source |
|---|---|---|
| `cfg-target-arch-count.txt` | `cfg(target_arch` outside `src/arch/` | `grep -rn 'cfg(target_arch' src --include='*.rs' \| grep -v '^src/arch/' \| wc -l` |
| `crate-mem-uses.txt` | `crate::mem::` use sites in `src/` | `grep -rn 'crate::mem::' src --include='*.rs' \| wc -l` |
| `crate-sched-uses.txt` | `crate::sched::` use sites in `src/` | `grep -rn 'crate::sched::' src --include='*.rs' \| wc -l` |
| `tcb-x86_64-capsules.txt` | lines of ring 0 code in the `microkernel-capsules` kernel, gated by `nonos-verify build` | `tools/nonos-tcb` after `make nonos-mk-capsules` |
| `tcb-x86_64-desktop-prod.txt` | the same for `nonos-mk-desktop-gui-prod`, the image that ships; reported, not gated in CI | `tools/nonos-tcb` after that build |

`src/mem/` is frozen. New `crate::mem::*` references are migration
regressions. The `crate::sched::` count is non-zero on purpose; new
growth is not.

When a refactor legitimately changes a count, update the file in the
same PR. The CI message prints the new actual count; copy it in.
Reviewers see the bump in the diff and accept or reject it.

Do not lower a baseline pre-emptively. The number must reflect HEAD;
otherwise the next push goes red.

The two TCB budgets count what rustc says it compiled into the kernel, not
the source tree, so other architectures and uncompiled modules are not in
them. They move one way: lower the file when ring 0 shrinks, so the next
growth is caught; a PR that raises one says why in its description.
