"""Item 6 mutants: each vendor rule broken once, the whole hda_proofs crate
run; the file put back after each. From the repo root."""
import subprocess
D="userland/capsule_driver_hda/src/controller/codec/"; C="userland/hda_proofs"
M=[
 ("V1 the HP rule for every vendor", D+"vendor_gpio.rs", "let realtek = (vendor_id >> 16) as u16 == VENDOR_REALTEK || vendor_id == 0x19e5_8326;", "let realtek = true;"),
 ("V2 the iMac GPIO2 rule dropped", D+"vendor_gpio.rs", "        0x1013_4206 | 0x1013_4207 if GPIO_23_MACS.contains(&subsystem_id) => Some(0x0c),\n", ""),
 ("V3 the NVIDIA Macs dropped", D+"vendor_gpio.rs", "apple || NVIDIA_MACS.contains(&subsystem_id)", "apple"),
 ("V4 the MacBook Air 7,2 dropped", D+"vendor_gpio.rs", "apple || subsystem_id == 0x8086_7270", "apple"),
 ("V5 GPIOs past the codec's own driven", D+"vendor_gpio.rs", ".filter(|&b| count >= 8 || b >> count == 0)", ""),
 ("C1 CS4206 given CS4208's sequence", D+"cirrus.rs", "0x1013_4206 | 0x1013_4207 => &CS420X,", "0x1013_4206 | 0x1013_4207 => &CS4208,"),
 ("C2 the DAC coefficient wrong", D+"cirrus_verbs.rs", "(0x11, COEF, 0x146a),", "(0x11, COEF, 0x002a),"),
 ("C6 an errata value wrong", D+"cirrus_verbs.rs", "(0x11, COEF, 0x2412),", "(0x11, COEF, 0x0412),"),
 ("C7 CS4208 coefficient wrong", D+"cirrus_verbs.rs", "(0x24, COEF, 0x1c01),", "(0x24, COEF, 0x0c01),"),
 ("C3 CS4208 not set up", D+"cirrus.rs", "0x1013_4208 => &CS4208,", "0x1013_4208 => &[],"),
 ("C4 coefficients sent as short verbs", D+"cirrus.rs", "let cmd = if verb <= 0xf {", "let cmd = if false {"),
 ("C5 cirrus never run", D+"vendor.rs", "    cirrus::init(link, codec)?;\n", ""),
 ("I1 power-map rule inverted", D+"idt.rs", "w.pin_cfg.connectivity() == CONN_NONE", "w.pin_cfg.connectivity() != CONN_NONE"),
 ("I2 92HD71 given 92HD83's ports", D+"idt.rs", "0x111d_7603 | 0x111d_7608 | 0x111d_76b0..=0x111d_76b7 => &HD71,", "0x111d_7603 | 0x111d_7608 | 0x111d_76b0..=0x111d_76b7 => &HD83,"),
 ("I3 idt never run", D+"vendor.rs", "    idt::init(link, codec)\n", "    Ok(())\n"),
 ("I4 the map sent to the root node", D+"idt.rs", "compose_verb(codec.cad, codec.afg, SET_POWER_MAP", "compose_verb(codec.cad, 0, SET_POWER_MAP"),
]
import sys
for name,f,old,new in [m for m in M if not sys.argv[1:] or m[0].split()[0] in sys.argv[1:]]:
    s=open(f).read(); assert s.count(old)==1,name; open(f,'w').write(s.replace(old,new))
    try: r=subprocess.run(f"cd {C} && timeout 900 cargo test --release 2>&1",shell=True,capture_output=True,text=True,timeout=1200).stdout
    finally: open(f,'w').write(s)
    fails=[l.split()[1].split('::')[-1] for l in r.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    comp="could not compile" in r
    print(f"| {name} | `{f.split('/')[-1]}` | {'KILLED' if fails or comp else 'SURVIVED'}{' (compile)' if comp and not fails else ''} | {', '.join(fails[:2])} |", flush=True)
