import subprocess
R="src/boot/handoff/types/refusal.rs"; RS="src/boot/handoff/types/refusal_says.rs"; KP="userland/kernel_proofs"; CP="userland/compositor_proofs"
N="userland/compositor/src/refusal_note.rs"
M=[
 ("K1 a stray bit trusted", R, "        _ if word >> 16 != 0 => FbRefusal::Unknown(word),\n", "", KP),
 ("K2 both flags accepted", R, "(true, true) => Err(Malformed),", "(true, true) => Ok(None),", KP),
 ("K3 no reason read as no GOP", R, "(false, false) => Ok(Some(FbRefusal::NoneGiven)),", "(false, false) => Ok(Some(FbRefusal::NoGop)),", KP),
 ("K4 a depth with a code that has none", R, "NO_FB_NO_GOP if bpp == 0 =>", "NO_FB_NO_GOP =>", KP),
 ("K5 an unknown word handed to userland raw", RS, "Self::Unknown(_) => WHY_UNKNOWN as u32,", "Self::Unknown(w) => w,", KP),
 ("K6 the loader's flag a different bit", "nonos-bootloader/src/handoff/types/constants.rs", "pub const FB_REFUSED: u64 = 1 << 16;", "pub const FB_REFUSED: u64 = 1 << 17;", KP),
 ("K7 the loader keeps a depth for any code", "nonos-bootloader/src/display/gop/refusal_word.rs", "let bpp = if code == NO_FB_BITMASK { bitmask_bpp as u32 } else { 0 };", "let bpp = bitmask_bpp as u32;", KP),
 ("K8 the both-flags check removed", "src/boot/handoff/api/security/framebuffer.rs", "    if super::super::super::types::refusal(handoff.flags, handoff.fb.pixel_format).is_err() {\n        return Err(reject(FbGeometryReason::RefusedAndAvailable));\n    }\n", "", KP),
 ("K9 ENOTSUP with no word written", "src/syscall/dispatch/router/graphics_backend.rs", "let told = out_why == 0 || write_user_value(out_why, &why).is_ok();", "let told = true;", KP),
 ("K10 the loader never sets the flag", "nonos-bootloader/src/handoff/prepare/flags.rs", "    } else {\n        f |= flags::FB_REFUSED;\n    }", "    }", KP),
 ("K11 flag_names stops at 16", "src/boot/handoff/types/constants.rs", "&NAMES[..(64 - flags.leading_zeros() as usize).min(17)]", "&NAMES[..(64 - flags.leading_zeros() as usize).min(16)]", KP),
 ("C1 said twice", N, "    if !said {", "    if !said || waited_ms < LOG_AFTER_MS {", CP),
 ("C2 the bound dropped", N, "!logged && waited_ms >= LOG_AFTER_MS", "!logged", CP),
 ("C3 logged before the bound", N, "!logged && waited_ms >= LOG_AFTER_MS", "!logged && waited_ms >= LOG_AFTER_MS / 2", CP),
 ("K4b BltOnly takes a depth", R, "NO_FB_BLT_ONLY if bpp == 0 =>", "NO_FB_BLT_ONLY =>", KP),
 ("K4c BadGeometry takes a depth", R, "NO_FB_BAD_GEOMETRY if bpp == 0 =>", "NO_FB_BAD_GEOMETRY =>", KP),
 ("K12 no 0 written with a framebuffer", "src/syscall/dispatch/router/graphics_backend.rs", ", (out_mm, mm), (out_why, 0)];", ", (out_mm, mm)];", KP),
 ("K13 a pointer left 0 written", "src/syscall/dispatch/router/graphics_backend.rs", "at != 0 && write_user_value(at, &v)", "write_user_value(at, &v)", KP),
 ("K14 out_why 0 written on the refusal path", "src/syscall/dispatch/router/graphics_backend.rs", "let told = out_why == 0 || write_user_value(out_why, &why).is_ok();", "let told = write_user_value(out_why, &why).is_ok();", KP),
 ("K15 refusal() drops the depth", "nonos-bootloader/src/display/gop/state.rs", "refusal_word(code, FB_BITMASK_BPP.load(Ordering::Acquire))", "refusal_word(code, 0)", KP),
 ("K16 refusal() reads no failure", "nonos-bootloader/src/display/gop/state.rs", "let code = FB_FAILURE.load(Ordering::Acquire);", "let code = 0u8;", KP),
]
for name,f,old,new,crate in M:
    s=open(f).read(); assert s.count(old)==1,name; open(f,'w').write(s.replace(old,new))
    try: r=subprocess.run(f"cd {crate} && cargo test --release 2>&1",shell=True,capture_output=True,text=True,timeout=1800).stdout
    finally: open(f,'w').write(s)
    fails=[l.split()[1].split('::')[-1] for l in r.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
    comp="could not compile" in r
    print(f"| {name} | `{f}` | {crate.split('/')[-1]} | {'KILLED' if fails or comp else 'SURVIVED'}{' (compile)' if comp and not fails else ''} | {', '.join(fails[:2])} |", flush=True)
