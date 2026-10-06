// Headless mirror of the page's exact logic: contexts rebuilt from displayed
// values, slot attribution, pad completeness, root reconstruction, and the
// adversarial cases a hostile bundle would try.
import { readFile } from "node:fs/promises";
const dir = new URL(".", import.meta.url).pathname;
const rd = f => readFile(dir + f);
const hex = b => [...b].map(x => x.toString(16).padStart(2, "0")).join("");
const unhex = h => new Uint8Array(h.match(/../g).map(x => parseInt(x, 16)));

const { instance } = await WebAssembly.instantiate(await rd("verifier.wasm"), {});
const w = instance.exports;
const index = JSON.parse(await readFile(dir + "index.json", "utf8"));
const policy = new Uint8Array(await rd("policy_root.bin"));
const kroot = new Uint8Array(await rd("kernel.root.bin"));
const push = b => { const p = w.wasm_alloc(b.length); new Uint8Array(w.memory.buffer, p, b.length).set(b); return p; };
const gate = (root, tr, ctx) => { const r = push(root), t = push(tr), c = push(ctx); const ok = w.verify(r, t, tr.length, c, ctx.length); w.wasm_free(r, root.length); w.wasm_free(t, tr.length); w.wasm_free(c, ctx.length); return ok === 1; };

const be = n => { const b = new Uint8Array(8); let v = BigInt(n); for (let i = 7; i >= 0; i--) { b[i] = Number(v & 0xffn); v >>= 8n; } return b; };
function buildContext(it, capsOverride) {
  const meas = unhex(it.measurement);
  const caps = capsOverride !== undefined ? capsOverride : it.caps;
  if (it.kind === "kernel") { const c = new Uint8Array(40); c.set(meas, 0); c.set(be(index.epoch), 32); return c; }
  const c = new Uint8Array(48); c.set(meas, 0); c.set(be(caps), 32); c.set(be(index.epoch), 40); return c;
}
const slotOf = t => { const d = t[8], off = 9 + d * 32; let i2 = 0; for (let i = 0; i < d; i++) i2 |= ((t[off + (i >> 3)] >> (i & 7)) & 1) << i; return i2; };
function anatomy(t) { const dv = new DataView(t.buffer, t.byteOffset, t.length); let o = 8; const depth = t[o]; o += 1 + depth * 32 + Math.ceil(depth / 8) + 64; const ood = dv.getUint32(o, true); o += 4 + ood * 16; const fl = dv.getUint32(o, true); o += 4 + fl * 32; const fin = dv.getUint32(o, true); o += 4 + fin * 16; return { depth, ood, fl, fin, fq: dv.getUint32(o, true) }; }

let ok = 0, bad = 0, anatBad = 0, slotBad = 0;
const capsules = index.items.filter(x => x.kind === "capsule");
for (const it of index.items) {
  const root = it.kind === "kernel" ? kroot : policy;
  const tr = new Uint8Array(await rd(it.slug + ".trailer.bin"));
  const v = gate(root, tr, buildContext(it));
  if (v) ok++; else { bad++; console.log("REFUSED", it.slug); }
  const a = anatomy(tr);
  if (a.depth !== 8 || a.fq !== 32 || a.fl < 10) { anatBad++; console.log("ANATOMY", it.slug, JSON.stringify(a)); }
  const expect = it.kind === "kernel" ? 0 : capsules.indexOf(it);
  if (slotOf(tr) !== expect) { slotBad++; console.log("SLOT", it.slug, slotOf(tr), "expected", expect); }
}

// reconstruction + completeness
const oL = w.wasm_alloc(32); w.reserved_leaf(oL);
const pad = hex(new Uint8Array(w.memory.buffer, oL, 32).slice()); w.wasm_free(oL, 32);
async function fold(f) { const l = new Uint8Array(await rd(f)); const p = push(l), o = w.wasm_alloc(32); const okf = w.fold_root(p, l.length / 32, o); const root = new Uint8Array(w.memory.buffer, o, 32).slice(); w.wasm_free(p, l.length); w.wasm_free(o, 32); return { okf: okf === 1, root, l }; }
const fp = await fold("leaves.bin"), fk = await fold("kernel_leaves.bin");
let padBad = 0;
for (let i = capsules.length; i < 256; i++) if (hex(fp.l.slice(i * 32, i * 32 + 32)) !== pad) padBad++;
for (let i = 1; i < 256; i++) if (hex(fk.l.slice(i * 32, i * 32 + 32)) !== pad) padBad++;
const recon = fp.okf && hex(fp.root) === index.policy_root && fk.okf && hex(fk.root) === hex(kroot) && padBad === 0;

// adversarial: display lies must refuse
const b = capsules.find(x => x.slug === "browser");
const btr = new Uint8Array(await rd("browser.trailer.bin"));
const lieCaps = gate(policy, btr, buildContext(b, b.caps | (1n && 1 << 20)));   // claim cap:grant
const lieMeas = gate(policy, btr, buildContext({ ...b, measurement: b.measurement.replace(/^../, "ff") }));
const ktr = new Uint8Array(await rd("kernel.trailer.bin")); ktr[ktr.length - 1] ^= 1;
const tamper = gate(kroot, ktr, buildContext(index.items[0]));

console.log(`verified ${ok}/${index.items.length}, refused ${bad}, anatomy bad ${anatBad}, slot bad ${slotBad}, pad bad ${padBad}`);
console.log(`reconstruction+completeness: ${recon ? "PROVEN" : "FAILED"}`);
console.log(`adversarial: lied caps ${lieCaps ? "ACCEPTED (BUG)" : "refused"}, lied measurement ${lieMeas ? "ACCEPTED (BUG)" : "refused"}, tampered proof ${tamper ? "ACCEPTED (BUG)" : "refused"}`);
process.exit(bad + anatBad + slotBad + padBad === 0 && recon && !lieCaps && !lieMeas && !tamper ? 0 : 1);
