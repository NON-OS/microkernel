# Hostile Linux guests, refused: 61ec29ea2

the isolation suite (`tools/nonos_isolation/hostile.steps`, README beside
it), booted on one development image. A Linux program in the Terminal tries
what a hostile app would: each attempt prints one `P <name> <result> <errno>`
line, and the screenshots are the record, since a Terminal run's output
never reaches serial.

- **Image:** `tools/nonos-dev-image --profile standard` of 61ec29ea2 (the
  branch at 072139db3, with MkGetPid on top), throwaway keys. Image sha256
  begins `112d39e7ab4a7ee7`.
- **Machine:** QEMU TCG, 4 vCPUs, Intel IOMMU lane, fresh disk, setup to the
  desktop. The 4-vCPU virtual machine was also running static checks and
  building at the time.
- **Steps:** `hostile.steps`, here as run.

## What each probe got

| Probe | What it tried | Result |
|---|---|---|
| ptrace | PTRACE_ATTACH to pid 1 | -1 EPERM: "a guest does not inspect or steer another" |
| vm_readv | process_vm_readv of 64 bytes of pid 1, with real iovecs | -1 EPERM: "no guest reads another's memory" |
| init_mod, finit_mod | load a kernel module | -1 ENOSYS (unserved) |
| kexec | load a new kernel | -1 ENOSYS |
| reboot | reboot with the magic numbers | -1 ENOSYS, and the machine runs on |
| mount | mount anything | -1 EPERM: "the tree is laid out by the personality" |
| bpf, perf | a BPF program, a perf event | -1 ENOSYS |
| nonos | syscall 1280, outside Linux's table | -1 ENOSYS |
| mk_debug | the native MkDebug, writing `NONOS-GUEST-PROBE` | -1 EPERM; the text is nowhere in `serial.log` |
| mk_kept | the native MkKeptPut on the installs' index | -1 EPERM |
| mmap_x | an anonymous read-write-execute mapping | -1 EPERM (W^X) |
| huge | an anonymous mapping of 1 TiB | -1 ENOMEM: "no memory to back it" |
| mprot_x | a read-write page at 0x30000000, then made executable | the map succeeds (805306368), the mprotect is -1 EPERM: "its policy does not grant JIT" |
| proc | the pids under /proc | only the run's own (2) |
| kill1, killall | signal pid 1; signal 0 to every pid | 0 signalled, ESRCH / 0 |
| home | /linux-home, /root/../../linux-home, /proc/kcore, /dev/mem | 0000; the path above the root is clamped |
| env1 | read /proc/1/environ | 0, ENOENT |
| tcp, udp | connect to 1.1.1.1:443, send to 8.8.8.8:53, with no NET_ANON | refused: "its policy does not grant NET_ANON or NET_LISTEN_LO" |
| raw, packet | a raw internet socket, a packet socket | 0: EACCES, EAFNOSUPPORT |
| forks | fork until refused, children waiting | stops at 511, the family's ceiling; `P alive after 511 forks` |

The personality's own tally after the calls run: "its policy refused 11:
jit=1 barred=3 unserved=7"; after the network run: "network=3".

## The serial log

- `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP-PROOF] cpu_count=4 PASS`,
  desktop at 129321 ms.
- 0 `[LOCK]` lines. No `[TRAP`, `[PANIC`, `[FATAL` or `panicked` line.
- 0 lines of guest output, and no program name. The only `[LINUX]` lines
  are the personality's own start-up ("personality up", "store settled",
  "personality ready").
- 64 `[DIAG] ipc.call unanswered` (62 `desktop_shell`, 2 `vfs_pool`) and 382
  slow TLB shootdowns, on a host running static checks and builds beside
  host with none (`../s1-lock/`). A run of these steps there is asked for.

## An earlier run that did not count

The first boot of these steps (on c9b18e2a1) called `syscall(@a)`. Perl's
`syscall` takes its first argument in scalar context, so it called the
syscall numbered by the length of `@a`: its "ptrace" was `fstat`, its
"mmap_x" was `poll`. Its calls screenshot is not evidence, and is not
here. The steps now pass the number on its own (02eb86f00). Its process,
network and fork probes, which use Perl's own built-ins, refused as above.

## Files

- `serial.log`: the whole boot.
- `calls.png`, `procs.png`, `net.png`, `forks.png`: the Terminal after each
  probe.
- `hostile.steps`: the steps as run.
