"""H-10 mutants: each rule broken once, the proof crate that must catch it run
whole. Run from the repo root; every file is put back after each run."""
import subprocess, sys
Q="src/hardware/broker/pci/quirk_bits/xhci_route.rs"; KP="userland/kernel_proofs"; XP="userland/xhci_proofs"
I="src/hardware/broker/pci/ident_read.rs"; C="src/hardware/broker/pci/companion.rs"
D="userland/capsule_driver_xhci/src/setup/"; P=D+"intel_ports_plan.rs"; IP=D+"intel_ports.rs"
M=[
 ("K1 USB2 mask ignored", Q, "XUSB2PR => Some(id.route.usb2prm),", "XUSB2PR => Some(0xFFFF),", KP),
 ("K2 USB3 mask ignored", Q, "USB3_PSSEN => Some(id.route.usb3prm),", "USB3_PSSEN => Some(0xFFFF),", KP),
 ("K3 device list dropped", Q, " || !SWITCHABLE.contains(&id.device)", "", KP),
 ("K4 vendor check dropped", Q, "id.vendor != INTEL || ", "", KP),
 ("K5 EHCI-claimed guard dropped", Q, "    if id.route.ehci_claimed {\n        return None; // another capsule's EHCI drives those ports\n    }\n", "", KP),
 ("K6 blocks: any bus", Q, "other == xhci && class", "class", KP),
 ("K7 blocks: any class", Q, " && class == 0x0C_0320 && claimed", " && claimed", KP),
 ("K8 blocks: unclaimed EHCI blocks too", Q, "class == 0x0C_0320 && claimed", "class == 0x0C_0320", KP),
 ("K9 masks not read from the device", I, "usb2prm: mask(0xD4)", "usb2prm: 0xFFFF", KP),
 ("K10 companion ignores claims", C, "let claimed = claim::claimed(h.device_id);", "let claimed = false;", KP),
 ("D1 order swapped", P, "[(USB3_PSSEN, m.usb3, m.usb3), (XUSB2PR, m.usb2, m.usb2)]", "[(XUSB2PR, m.usb2, m.usb2), (USB3_PSSEN, m.usb3, m.usb3)]", XP),
 ("D2 no restore on teardown", D+"teardown.rs", "            ports.restore();\n", "            let _ = ports;\n", XP),
 ("D3 restore leaves bits set", P, "[(XUSB2PR, m.usb2, 0), (USB3_PSSEN, m.usb3, 0)]", "[(XUSB2PR, m.usb2, m.usb2), (USB3_PSSEN, m.usb3, m.usb3)]", XP),
 ("D4 restore before the halt", D+"teardown.rs", "        let _ = quiesce(self.layout.op_base);\n        // Halted, so nothing it runs follows the ports; the claim still held.\n        if let Some(ports) = self.ports.take() {\n            ports.restore();\n        }\n", "        if let Some(ports) = self.ports.take() {\n            ports.restore();\n        }\n        let _ = quiesce(self.layout.op_base);\n", XP),
 ("D5 switch after the controller runs", D+"sequence.rs", "        driver.ports = switch(dev.device_id, claim_epoch, dev.vendor);\n    }\n    program(&mut driver);\n    start_checked(&mut driver)?;\n", "    }\n    program(&mut driver);\n    start_checked(&mut driver)?;\n    driver.ports = switch(dev.device_id, claim_epoch, dev.vendor);\n", XP),
 ("D6 Sony VAIO switched", P, " && subsystem != CANNOT_SWITCH", "", XP),
 ("D7 vendor check dropped", P, "vendor == 0x8086 && ", "", XP),
 ("D8 zero mask written", IP, "switch_steps(masks).into_iter().filter(|s| s.1 != 0)", "switch_steps(masks).into_iter()", XP),
 ("D9 refused switch kept", IP, "(moved || rc >= 0, problems || !ok)", "(true, problems || !ok)", XP),
 ("D10 read-back unchecked", IP, "let ok = rc >= 0 && took(read(at, 2) as u16, mask, value);", "let ok = rc >= 0;", XP),
]
for name,f,old,new,crate in M[int(sys.argv[1]) if len(sys.argv) > 1 else 0:]:
    s=open(f).read(); assert s.count(old)==1,name; open(f,'w').write(s.replace(old,new))
    try: r=subprocess.run(f"cd {crate} && cargo test --release 2>&1",shell=True,capture_output=True,text=True,timeout=2400).stdout
    finally: open(f,'w').write(s)
    fails=[l.split()[1].split('::')[-1] for l in r.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    comp="could not compile" in r
    print(f"| {name} | `{f}` | {crate.split('/')[-1]} | {'KILLED' if fails or comp else 'SURVIVED'}{' (compile)' if comp and not fails else ''} | {', '.join(fails[:2])} |", flush=True)
