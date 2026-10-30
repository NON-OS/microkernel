# Terminal 05: the prompt's branch and jobs, booted 25 October 2026

The prompt shows the git branch and the background jobs, each only when it
says something (e097a7cb4e), in an image built at ddbe96d765.

## How it ran

- Image: `make dev-image` at ddbe96d765 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys.

## What it showed

- `1-git-init-branch-main.png`: `cd workspace` and `git init`: the prompt after
  them reads `nonos@nonos ~/workspace main %`; before it, `~/workspace %`.
- `2-branch-and-one-job.png`: `curl https://example.org &` through the Nym
  mixnet, `jobs` says `[1] Running`, and the prompt reads
  `~/workspace main 1 job %`. `ping 10.0.2.2 &` was refused on the spot (Nym
  chosen), so it ran no job and the prompt counted none.
