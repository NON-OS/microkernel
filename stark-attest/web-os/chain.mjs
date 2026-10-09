// The whole boot chain, from the shipping artifacts alone. No fixture roots:
// the kernel image supplies its own body and embedded proof through the
// NONOSIMG footer, the bootloader binary supplies the root it enforces, and
// the wasm gate decides. This is the flow the drop-zone runs in a browser.
import { readFile } from "node:fs/promises";
const dir = new URL(".", import.meta.url).pathname;
const hex = b => [...b].map(x => x.toString(16).padStart(2, "0")).join("");

const { instance } = await WebAssembly.instantiate(await readFile(dir + "verifier.wasm"), {});
const w = instance.exports;
const push = b => { const p = w.wasm_alloc(b.length); new Uint8Array(w.memory.buffer, p, b.length).set(b); return p; };
const free = (p, n) => w.wasm_free(p, n);

function b3(bytes) {
  const p = push(bytes), o = w.wasm_alloc(32);
  w.blake3_hash(p, bytes.length, o);
  const d = new Uint8Array(w.memory.buffer, o, 32).slice();
  free(p, bytes.length); free(o, 32); return d;
}
function gate(root, tr, ctx) {
  const r = push(root), t = push(tr), c = push(ctx);
  const ok = w.verify(r, t, tr.length, c, ctx.length);
  free(r, 32); free(t, tr.length); free(c, ctx.length); return ok === 1;
}

// 1. the shipping image: parse the NONOSIMG footer
const img = new Uint8Array(await readFile("/Users/ek/nonos-launchpad/target/kernel_attested.bin"));
const f = img.slice(img.length - 64);
const dv = new DataView(f.buffer, f.byteOffset);
const magic = new TextDecoder().decode(f.slice(0, 8));
if (magic !== "NONOSIMG") throw new Error("not a NONOS image");
const kOff = dv.getUint32(24, true), kSize = dv.getUint32(28, true);
const pOff = dv.getUint32(40, true), pSize = dv.getUint32(44, true);
const rollback = dv.getUint32(56, true);
const kernel = img.slice(kOff, kOff + kSize);
const trailer = img.slice(pOff, pOff + pSize);
console.log(`image: ${img.length} bytes, kernel body ${kSize}, embedded trailer ${pSize}, rollback index ${rollback}`);
console.log(`trailer magic in image: ${new TextDecoder().decode(trailer.slice(0, 8))}`);

// 2. the bootloader binary: pull out the root it enforces
const efi = new Uint8Array(await readFile("/Users/ek/nonos-launchpad/nonos-bootloader/target/x86_64-unknown-uefi/release/nonos_boot.efi"));
const known = new Uint8Array(await readFile(dir + "kernel.root.bin"));
let hits = 0, at = -1;
outer: for (let i = 0; i <= efi.length - 32; i++) {
  for (let j = 0; j < 32; j++) if (efi[i + j] !== known[j]) continue outer;
  hits++; at = i;
}
console.log(`boot root found in nonos_boot.efi: ${hits} occurrence(s) at offset ${at}`);
const bootRoot = efi.slice(at, at + 32);

// 3. measure the body in-wasm, build the boot context, run the gate
const meas = b3(kernel);
const ctx = new Uint8Array(40); ctx.set(meas, 0); ctx[39] = 1; // epoch 1 big-endian
const ok = gate(bootRoot, trailer, ctx);
console.log(`kernel measurement: ${hex(meas).slice(0, 24)}…`);
console.log(`STARK from the image, root from the bootloader: ${ok ? "VERIFIED" : "REFUSED"}`);

// 4. the policy root the kernel enforces on capsules, extracted from the body
const policy = new Uint8Array(await readFile(dir + "policy_root.bin"));
let phits = 0;
outer2: for (let i = 0; i <= kernel.length - 32; i++) {
  for (let j = 0; j < 32; j++) if (kernel[i + j] !== policy[j]) continue outer2;
  phits++;
}
console.log(`capsule policy root embedded in the kernel body: ${phits} occurrence(s)`);

// 5. tamper: flip one bit of the kernel body, remeasure, verify
const evil = kernel.slice(); evil[1000000] ^= 1;
const ectx = new Uint8Array(40); ectx.set(b3(evil), 0); ectx[39] = 1;
const eok = gate(bootRoot, trailer, ectx);
console.log(`one flipped kernel bit: ${eok ? "ACCEPTED (BUG)" : "refused"}`);

process.exit(ok && hits === 1 && phits >= 1 && !eok ? 0 : 1);
