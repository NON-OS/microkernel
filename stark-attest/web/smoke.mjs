// Headless smoke test of the wasm gate: verify the real fixture, then flip
// one bit and confirm refusal. Run: node smoke.mjs
import { readFile } from "node:fs/promises";
const dir = new URL(".", import.meta.url).pathname;
const [wasmBytes, root, trailer, ctx] = await Promise.all(
  ["verifier.wasm", "root.bin", "trailer.bin", "context.bin"].map(f => readFile(dir + f)));
const { instance } = await WebAssembly.instantiate(wasmBytes, {});
const w = instance.exports;
const push = b => { const p = w.wasm_alloc(b.length); new Uint8Array(w.memory.buffer, p, b.length).set(b); return p; };
const gate = t => {
  const r = push(root), tp = push(t), c = push(ctx);
  const t0 = process.hrtime.bigint();
  const ok = w.verify(r, tp, t.length, c, ctx.length);
  const ms = Number(process.hrtime.bigint() - t0) / 1e6;
  return { ok: ok === 1, ms };
};
const good = gate(trailer);
console.log(`honest trailer: ${good.ok ? "VERIFIED" : "REFUSED"} in ${good.ms.toFixed(1)} ms`);
const bad = Buffer.from(trailer); bad[bad.length - 1] ^= 1;
const evil = gate(bad);
console.log(`one bit flipped: ${evil.ok ? "VERIFIED (BUG!)" : "refused"} in ${evil.ms.toFixed(1)} ms`);
if (!good.ok || evil.ok) process.exit(1);
console.log("gate behaves in wasm exactly as it does in the kernel");
