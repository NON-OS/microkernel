# Method: how the numbers were measured

Every number in this review comes from one of the scripts on this page, run against a clean checkout of `551bbd34`. The scripts read only. To regenerate a figure, save the script, check out the commit and run it as shown.

## Counting lines

Two counts are given everywhere a size matters.

- **phys**: physical lines, what `wc -l` prints. This is the number most project documents use.
- **code**: physical lines minus blank lines, lines that hold only a `//` comment, and `/* ... */` comment blocks. Every source file in `src/` opens with the same fifteen-line AGPL header, so across 7,676 files about 115,000 of the 396,916 phys lines are licence text. code lines are the fairer measure of what a reviewer or a prover has to read.

Only `*.rs` and `*.S` files under `src/` are counted. The 9 `.inc` assembler includes, 9 `.h` headers and 2 `.c` files under `src/` are left out. Seven of the includes belong to `src/arch/x86_64/asm/`; the C files and three headers are the PQClean support in `src/crypto/pqclean_support/`, and `build.rs:42-43` also compiles ML-KEM and ML-DSA from `third_party/pqclean/` into the kernel. That C code is part of today's ring 0 and is counted separately in [13-migration-plan.md](13-migration-plan.md).

### Script 1: per-subtree totals (`measure.sh`)

```sh
#!/bin/sh
# Usage: measure.sh <repo-root>
# Physical lines of *.rs, *.S and *.s under src/, per subtree to depth 4.
root=${1:-.}; cd "$root" || exit 1
echo "# total"
find src -type f \( -name '*.rs' -o -name '*.S' -o -name '*.s' \) | xargs cat | wc -l
find src -type f -name '*.rs' | wc -l
echo "# per subtree: lines files path"
find src -type f \( -name '*.rs' -o -name '*.S' -o -name '*.s' \) -print0 | xargs -0 wc -l | grep -v ' total$' |
awk '{n=$1; p=$2; split(p,a,"/"); d=""; for(i=2;i<length(a) && i<=5;i++){ d=(d==""?a[i]:d"/"a[i]); L[d]+=n; F[d]++ } if(length(a)==2){L["(root files)"]+=n;F["(root files)"]++}}
END{for(k in L) printf "%d %d %s\n", L[k], F[k], k}' | sort -k3
```

At `551bbd34` it prints a total of 396,916 lines (`*.rs` and `*.S`) in 7,648 `.rs` files plus 28 `.S` files. The commonly quoted figure of about 395K lines of Rust in 7,648 files is the `.rs` part of the same count.

### Script 2: classification totals (`classify.py`)

The classification of [04-classification.md](04-classification.md) is a table of path prefixes under `src/`, each with an outcome class and a destination. The table is printed in full in the appendix of that document; save it as `classes.tsv`. Each file takes the row of its longest matching prefix.

```python
#!/usr/bin/env python3
"""Sum lines of src/**/*.rs and src/**/*.S by classification.

Usage: python3 -I classify.py <repo-root> <classes.tsv> [--dirs]

Two counts per file:
  phys  physical lines, what `wc -l` prints;
  code  lines left after removing blank lines, lines that hold only a //
        comment (Rust) or a # / // comment (assembly), and /* ... */ blocks.
Each file takes the class of the longest prefix in classes.tsv that matches
its path under src/ by whole components. Under src/hardware, a *_capsule
directory with no row of its own is U:init, except its client/ and protocol/
subdirectories, which are D unless the capsule is one of LIVE_CLIENTS.
"""
import os, sys, collections

rules = {}

def load(table):
    for line in open(table, encoding='utf-8'):
        if not line.strip() or line.startswith('#'):
            continue
        p, c, d = line.rstrip('\n').split('\t')
        rules[p] = (c, d)

LIVE_CLIENTS = {'nvme_capsule', 'ahci_capsule', 'virtio_blk_capsule', 'usb_msc_capsule'}

def classify(rel):
    parts = rel.split('/')
    best = None
    for i in range(len(parts), 0, -1):
        k = '/'.join(parts[:i])
        if k in rules:
            best = (k, rules[k])
            break
    if parts[0] == 'hardware' and len(parts) > 2 and parts[1].endswith('_capsule'):
        if best is None or '/' not in best[0]:
            if parts[1] in LIVE_CLIENTS:
                return ('U', 'storage' if parts[2] in ('client', 'protocol') else 'init')
            if parts[2] in ('client', 'protocol'):
                return ('D', 'dead')
            return ('U', 'init')
    return best[1] if best else ('?', '?')

def counts(path):
    phys = code = 0
    in_block = False
    for raw in open(path, encoding='utf-8', errors='replace'):
        phys += 1
        s = raw.strip()
        if in_block:
            if '*/' in s:
                in_block = False
                s = s.split('*/', 1)[1].strip()
                if not s:
                    continue
            else:
                continue
        if s.startswith('/*'):
            if '*/' not in s[2:]:
                in_block = True
            continue
        if not s or s.startswith('//') or (path.endswith('.S') and s.startswith('#') and not s.startswith('#include') and not s.startswith('#define')):
            continue
        code += 1
    return phys, code

def main():
    root, table = sys.argv[1], sys.argv[2]
    load(table)
    phys = collections.Counter(); code = collections.Counter(); files = collections.Counter()
    dest = collections.Counter(); by = collections.Counter()
    for dp, dn, fn in os.walk(os.path.join(root, 'src')):
        for f in fn:
            if not (f.endswith('.rs') or f.endswith('.S')):
                continue
            full = os.path.join(dp, f)
            rel = os.path.relpath(full, os.path.join(root, 'src'))
            p, c = counts(full)
            cls, d = classify(rel)
            phys[cls] += p; code[cls] += c; files[cls] += 1; dest[(cls, d)] += p
            parts = rel.split('/')
            by[(cls, '/'.join(parts[:2]) if len(parts) > 2 else parts[0])] += p

    print('class phys code files')
    for k in 'KRUDX?':
        if files[k]:
            print(k, phys[k], code[k], files[k])
    print('total', sum(phys.values()), sum(code.values()), sum(files.values()))
    print('x86_64-compiled (all but X)', sum(v for k, v in phys.items() if k != 'X'),
          sum(v for k, v in code.items() if k != 'X'))
    print('\nby class and destination (phys)')
    for (k, d), n in sorted(dest.items(), key=lambda x: (x[0][0], -x[1])):
        print(k, d, n)
    if '--dirs' in sys.argv:
        print('\nby class and subtree, 800 phys lines or more')
        for (k, s), n in sorted(by.items(), key=lambda x: (x[0][0], -x[1])):
            if n >= 800:
                print(k, s, n)

if __name__ == '__main__':
    main()
```

Run as `python3 -I classify.py <repo-root> classes.tsv --dirs`. At `551bbd34` it prints:

```
class phys code files
K 134781 68469 2623
R 51949 27531 935
U 96019 49693 1926
D 67067 41054 1161
X 47100 22897 1031
total 396916 209644 7676
x86_64-compiled (all but X) 349816 186747
```

### Script 3: per-module table (`bymodule.py`)

```python
#!/usr/bin/env python3
"""Per top-level module of src/: phys and code lines, files, phys lines per class.
Usage: python3 -I bymodule.py <repo-root> <classes.tsv>  (classify.py beside it)"""
import os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import classify as C
root = sys.argv[1]
C.load(sys.argv[2])
row = collections.defaultdict(collections.Counter)
for dp, dn, fn in os.walk(os.path.join(root, 'src')):
    for f in fn:
        if not (f.endswith('.rs') or f.endswith('.S')):
            continue
        full = os.path.join(dp, f)
        rel = os.path.relpath(full, os.path.join(root, 'src'))
        top = rel.split('/')[0] if '/' in rel else '(crate root)'
        p, c = C.counts(full)
        k, _ = C.classify(rel)
        r = row[top]; r['phys'] += p; r['code'] += c; r['files'] += 1; r[k] += p
print('| module | phys | code | files | K | R | U | D | X |')
print('|---|---:|---:|---:|---:|---:|---:|---:|---:|')
for top, r in sorted(row.items(), key=lambda x: -x[1]['phys']):
    print(f"| {top} | {r['phys']} | {r['code']} | {r['files']} | " + ' | '.join(str(r[k]) for k in 'KRUDX') + ' |')
```

Its output is the table in [01-current-state.md](01-current-state.md).

### Script 4: the ring-0 budget (`budget.py`)

The target of [13-migration-plan.md](13-migration-plan.md) is a sum of per-area budgets. The script checks that the areas add up to today's K and R totals from `classify.py` and prints the table that document shows.

```python
#!/usr/bin/env python3
"""Ring-0 budget: today's K and R physical lines per area (from classify.py),
and the target for each area after the migration. Prints a markdown table."""
rows = [
 # (area, K today, R today, target, basis of the target)
 ("arch: x86_64 privileged core and arch-neutral layers", 33923, 0, 33923, "unchanged"),
 ("arch: ACPI parser and tables, hypervisor, duplicate IDT, time", 0, 18536, 5450, "fixed-table reader 2,000; SynIC and hypercall setup 800; time 2,500; ACPI glue 150; duplicate IDT deleted"),
 ("boot", 3044, 0, 3044, "unchanged"),
 ("bus", 0, 1181, 0, "enumeration in drivers/pci; BAR assignment and AER to devmgr"),
 ("capabilities", 1327, 0, 2500, "object table, derivation tree, classes"),
 ("crypto (RNG, entropy, constant time, core)", 3797, 0, 3797, "unchanged"),
 ("drivers/pci", 7196, 0, 6200, "statistics and duplicate tables removed"),
 ("entry, crate root, time, log", 1689, 0, 1689, "unchanged"),
 ("hardware/broker", 16189, 11018, 11000, "mechanism only (10-target-device-authority.md)"),
 ("interrupts", 5236, 0, 5236, "unchanged"),
 ("ipc", 1511, 0, 1800, "endpoints, notifications, capability transfer"),
 ("kernel_core: init and stacks", 2509, 0, 2509, "unchanged"),
 ("kernel_core: capsule_spawn", 0, 2765, 600, "verdict, pinned table, measurement at start, flat init image"),
 ("kernel_core: surface_registry", 0, 4637, 0, "replaced by MemoryObject, counted under memory"),
 ("memory", 20285, 2193, 21835, "live allocators and paging; MemoryObject and page-table pool 800; sealed-page digests 150; guard and protection residue 600"),
 ("process: core, scheduler, exit, entry, caps", 10618, 0, 10618, "unchanged"),
 ("process/foreign", 0, 4842, 1500, "syscall-redirect and fault endpoints, register read and write, cross-space copy"),
 ("process/signal", 0, 2057, 150, "fault kind to exit code"),
 ("sched", 92, 0, 92, "unchanged"),
 ("security: hardening, ceiling, observability, boot session, random", 3055, 0, 3055, "unchanged"),
 ("security: attest_registry, zerostate", 0, 1166, 700, "append-only measurement log and root 400; RAM wipe 300"),
 ("security: PCR 14 extend at locality 1 (new)", 0, 0, 300, "one TPM command path, from today's 5,674-line driver"),
 ("smp", 5216, 0, 5216, "unchanged"),
 ("sys", 5203, 941, 4800, "duplicate APIC init merged (-700); clock-page writer 300"),
 ("syscall", 12995, 2613, 5500, "46 calls: handlers 4,000, decode and check 1,500"),
 ("usercopy", 896, 0, 896, "unchanged"),
]
K = sum(r[1] for r in rows); R = sum(r[2] for r in rows); T = sum(r[3] for r in rows)
print("| Area | K today | R today | Target | Basis |")
print("|---|---:|---:|---:|---|")
for a, k, r, t, b in rows:
    print(f"| {a} | {k:,} | {r:,} | {t:,} | {b} |")
print(f"| **Total** | **{K:,}** | **{R:,}** | **{T:,}** | |")
print(f"\ncheck: K={K} (classify.py 134781), R={R} (classify.py 51949)")
print(f"target code lines at today's K ratio 68469/134781: {round(T*68469/134781):,}")
```

## Counting system calls

```sh
grep -c '^\[desc\.' abi/syscalls.toml                                   # 196
awk '/^\[numbers\]/{f=1;next} /^\[/{f=0} f && /=/' abi/syscalls.toml | wc -l   # 196
grep -c 'caps_any' abi/syscalls.toml                                    # 2
```

The router was checked against the table by listing the enum variants of `src/syscall/numbers/defs.rs` and the variants named in `src/syscall/dispatch/router/` and comparing the two sets with `comm`; both hold the same 196 tags. Family counts in [02-syscall-surface-today.md](02-syscall-surface-today.md) come from the per-family number files under `src/syscall/microkernel/numbers/` and the router's crypto, admin and graphics arms.

## Finding dead code

"Dead" in this review means no path from a live entry point reaches the code; "inert" means a path reaches it and it has no effect (a getter whose setter is never called, a structure built and never read). It was established by grep, in three steps, and every dead verdict that a classification depends on was checked by hand:

1. For a module `m`, search for `crate::<path>::m`, `super::m` and every name `m` re-exports, outside `m`'s own directory.
2. For each hit, decide whether the calling code is itself live, recursively, back to `kernel_main`, a syscall arm, an interrupt vector or the boot sequence in `src/boot/main/core_init/`.
3. For glob re-exports (`pub use x::*` in `src/arch/mod.rs`, `src/security/mod.rs`, `src/crypto/mod.rs`), search for the exported names one by one.

Grep cannot see paths built by macros. The dead list is therefore strong evidence and not compiler proof; [13-migration-plan.md](13-migration-plan.md) makes "delete, then build every profile" the first step of each deletion, which turns each verdict into a compiler-checked one.

The single live ELF load path was found this way:

```sh
grep -rn 'load_elf_entry_into\|load_elf_executable_into\|load_elf_executable\b' src --include=*.rs | grep -v '^src/elf/'
grep -rn 'crate::elf::' src --include=*.rs | grep -v '^src/elf/'
```

The first finds one call, `src/kernel_core/process_spawn/capsule_spawn/runner/install/load_elf_into_pid.rs:34`, plus 63 `debug_tag` strings that only name the function in log text. The second finds that call's import and `crate::elf::loader::init_elf_loader()` at `src/kernel_core/init/entry/init_runtime.rs:45`.

## Checking citations

Every `path:line` in these documents was checked by this script, which fails on a missing file or a line past the end of the file. Line content was checked by reading each cited line while writing.

```python
#!/usr/bin/env python3
"""Check every file:line citation in the architecture documents.

Usage: python3 -I checkcites.py <repo-root> <docs-dir>

A citation is a repository-relative path with a recognised extension,
followed by :N, :N-M or :N,M,... Each named file must exist at the
checked-out commit and each cited line must be within the file. Prints
every failure and a summary; exits 1 on any failure."""
import os, re, sys
root, docs = sys.argv[1], sys.argv[2]
pat = re.compile(r'((?:[A-Za-z0-9_.\-]+/)*[A-Za-z0-9_.\-]+\.(?:rs|S|toml|md|json|py|sh|mk|nix|lean|inc|ld|txt|bin|rs\.in)):(\d+(?:-\d+)?(?:,\d+(?:-\d+)?)*)')
fails = 0; seen = 0; files = set()
for dp, dn, fn in os.walk(docs):
    for f in fn:
        if not f.endswith('.md'):
            continue
        doc = os.path.join(dp, f)
        for no, line in enumerate(open(doc, encoding='utf-8'), 1):
            for m in pat.finditer(line):
                path, spec = m.group(1), m.group(2)
                full = os.path.join(root, path)
                seen += 1
                if not os.path.isfile(full):
                    print(f'{doc}:{no}: missing file {path}'); fails += 1; continue
                files.add(path)
                n = sum(1 for _ in open(full, 'rb'))
                for part in spec.split(','):
                    a, _, b = part.partition('-')
                    hi = int(b or a)
                    if int(a) < 1 or hi > n or int(a) > hi:
                        print(f'{doc}:{no}: {path}:{part} out of range (file has {n} lines)'); fails += 1
print(f'{seen} citations, {len(files)} distinct files, {fails} failures')
sys.exit(1 if fails else 0)
```

Run as `python3 -I checkcites.py <repo-root> docs/architecture`.

## The audit repository

The findings were read from `NON-OS/auditing` at commit `0f5e792` (cloned read-only with `GIT_LFS_SKIP_SMUDGE=1 git clone --depth 1`). Its `findings/published/README.md` lists 16 published findings, all open. The audit's own locations name commit `b3dd1fa37ce2`; every location this review relies on was re-read at `551bbd34`, and where a line moved, the `551bbd34` line is cited.
