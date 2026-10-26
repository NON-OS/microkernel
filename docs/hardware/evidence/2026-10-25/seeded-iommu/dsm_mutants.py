"""Item 7 _DSM mutants: each rule of the i2c-hid _DSM matcher broken once,
the whole acpi_aml_proofs crate run; the file put back after each."""
import subprocess
F="src/arch/x86_64/acpi/devices/i2c/dsm.rs"; C="userland/acpi_aml_proofs"
M=[
 ("A1 a register of 0 taken", "n != 0 && n <= u64::from(u16::MAX)", "n <= u64::from(u16::MAX)"),
 ("A2 a value past 16 bits taken", "n != 0 && n <= u64::from(u16::MAX)", "n != 0"),
 ("A3 another GUID's branch trusted", "if find(device_body, &HID_I2C_DSM_GUID).is_some() && !other_uuid_literal {", "if !other_uuid_literal {"),
 ("A4 One == Arg2 not read", " || (w[1] == ONE_OP && w[2] == ARG2)", ""),
]
for name,old,new in M:
    s=open(F).read(); assert s.count(old)==1,name; open(F,'w').write(s.replace(old,new))
    try: r=subprocess.run(f"cd {C} && timeout 1500 cargo test --release 2>&1",shell=True,capture_output=True,text=True,timeout=1800).stdout
    finally: open(F,'w').write(s)
    fails=[l.split()[1].split('::')[-1] for l in r.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    comp="could not compile" in r
    print(f"| {name} | `{F}` | {'KILLED' if fails or comp else 'SURVIVED'}{' (compile)' if comp and not fails else ''} | {', '.join(fails[:2])} |", flush=True)
