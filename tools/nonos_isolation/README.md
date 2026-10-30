# Hostile Linux guests

`hostile.steps` drives a fresh development image (`tools/nonos-drive
--steps tools/nonos_isolation/hostile.steps`) to the desktop, opens the
Terminal and runs Linux programs that try what a hostile app would. Every
attempt must be refused, and the machine must still answer afterwards. Each
probe prints one `P <name> <result> <errno>` line per attempt, and the
screenshot after each run is its record. A Terminal run's output reaches
its own Terminal only, never the serial console, so the screenshots are
the evidence, beside the serial log's kernel lines.

The probes use programs every image already carries (Perl, through
`syscall()`, reaches any raw Linux call), so nothing hostile is added to an
image. Perl's `syscall` takes its first argument in scalar context, so the
number is passed on its own: `syscall(@a)` would call the syscall numbered
by the length of `@a`, which the first version of these steps did.

| Probe | What it tries | Refused when |
|---|---|---|
| `calls` ptrace | PTRACE_ATTACH to pid 1 | -1 |
| `calls` vm_readv | process_vm_readv of 64 bytes of pid 1, with real iovecs | -1 |
| `calls` init_mod, finit_mod | load a kernel module | -1 |
| `calls` kexec | load a new kernel | -1 |
| `calls` reboot | reboot with the magic numbers | -1, and the machine runs on |
| `calls` mount | mount anything | -1 |
| `calls` bpf, perf | a BPF program, a perf event | -1 |
| `calls` nonos | syscall 1280, outside Linux's table | -1 (ENOSYS) |
| `calls` mk_debug | the native MkDebug (`MDBG`), writing `NONOS-GUEST-PROBE` to serial | -1, and the text is nowhere in the serial log |
| `calls` mk_kept | the native MkKeptPut (`MKPT`) on the installs' index | -1 |
| `calls` mmap_x | an anonymous read-write-execute mapping, no JIT in the box | -1 |
| `calls` huge | an anonymous mapping of 1 TiB | -1 (ENOMEM), and the machine runs on |
| `calls` mprot_x | map a read-write page at 0x30000000, then make it executable | the map succeeds, the mprotect is -1 |
| `procs` proc | the pids under /proc | only the run's own family |
| `procs` kill1, killall | signal pid 1; signal 0 to every pid | 0 signalled / only its family |
| `procs` home | /linux-home, /root/../../linux-home, /proc/kcore, /dev/mem exist | 0000 |
| `procs` env1 | read /proc/1/environ | 0 |
| `net` tcp, udp | connect to 1.1.1.1:443, send to 8.8.8.8:53, in a box without NET_ANON | refused |
| `net` raw, packet | a raw internet socket, a packet socket | 0 |
| `forks` | fork until refused, children waiting | stops at the family's ceiling, and `P alive after <n> forks` prints after |
| `after` | `linux echo still here` | it prints |

The results of a boot are in `docs/release-evidence/0.9.3/isolation-*`.
