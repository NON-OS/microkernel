"""Item 7 mutants: each rule the DMAR and IVRS sweeps hold broken once, the
whole kernel_proofs crate run; the file put back after each. From the repo root."""
import subprocess, sys
O="src/arch/x86_64/acpi/parser/other/"; KP="userland/kernel_proofs"
M=[
 ("S1 an inverted IVHD range kept", O+"ivhd_kind.rs", ".filter(|&first| first <= id).and_then(|f| one(f, id, false))", ".and_then(|f| one(f, id, false))"),
 ("S2 DMAR padding read as a cut", O+"dmar_walk.rs", "let padding = table[at..].iter().all(|&b| b == 0);", "let padding = false;"),
 ("S3 DMAR hands on a cut structure", O+"dmar_walk.rs", "table.get(at..at + length).filter(|_| length >= 4)", "table.get(at..(at + length).min(table.len())).filter(|_| length >= 4)"),
 ("S4 RMRR inverted region kept", O+"dmar_rmrr.rs", " && limit >= base;", ";"),
 ("S5 ANDD empty name kept", O+"dmar_andd.rs", "if len == 0 || len > MAX_NAME", "if len > MAX_NAME"),
 ("S6 IVRS base counted twice", O+"ivrs_walk.rs", "if base != 0 && !out[..found].contains(&base) {", "if base != 0 {"),
 ("S7 IVRS short IVHD read for a base", O+"ivrs_walk.rs", "if IVHD_TYPES.contains(&kind) && length >= 24 {", "if IVHD_TYPES.contains(&kind) && length >= 16 {"),
 ("S8 IVMD empty range kept", O+"ivmd_block.rs", "if len == 0 || last < first || end <= start {", "if len == 0 || last < first {"),
 ("S9 inverted alias kept", O+"ivhd_alias.rs", "match found.filter(|a| a.first <= a.last) {", "match found {"),
 ("S10 special counted twice", O+"ivhd_special.rs", "match Some(s.id).filter(|id| !out[..n].contains(id)) {", "match Some(s.id) {"),
]
for name,f,old,new in M[int(sys.argv[1]) if len(sys.argv) > 1 else 0:]:
    s=open(f).read(); assert s.count(old)==1,name; open(f,'w').write(s.replace(old,new))
    try: r=subprocess.run(f"cd {KP} && timeout 1500 cargo test --release 2>&1",shell=True,capture_output=True,text=True,timeout=1800).stdout
    finally: open(f,'w').write(s)
    fails=[l.split()[1].split('::')[-1] for l in r.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    comp="could not compile" in r; hung="timed out" in r or r.strip()==""
    print(f"| {name} | `{f}` | {'KILLED' if fails or comp else 'SURVIVED'}{' (compile)' if comp and not fails else ''} | {', '.join(fails[:2])} |", flush=True)
