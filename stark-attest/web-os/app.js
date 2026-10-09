"use strict";
// The attestation surface: parse each proof, show its real structure, verify it
// in the same gate the kernel runs. A verdict without the work behind it is a
// checkbox; this shows the work.

const $ = id => document.getElementById(id);
const hex = b => [...b].map(x => x.toString(16).padStart(2, "0")).join("");
const short = (h, n = 10) => h.slice(0, n) + "…" + h.slice(-4);

// capability bits, from src/security/policy/capability/types.rs
const CAPS = ["process:create","process:kill","memory:map","memory:unmap","file:read","file:write",
"file:create","file:delete","net:bind","net:connect","device","syscall","irq","module:load",
"module:unload","crypto:keys","vault","ephemeral-mem","isolation","zerostate","cap:grant","cap:revoke",
"attest:create","attest:verify","secure-boot","crypto-fs-vault","pq-signatures","hw-abstraction","debug","audit"];
const decodeCaps = n => { const o = []; for (let i = 0; i < 32; i++) if ((n >>> i) & 1) o.push(CAPS[i] || ("bit" + i)); return o; };

let wasm, mem, index, policyRoot, kernelRoot;
const times = [];
const cache = {};

async function bytes(p) { const r = await fetch(p); if (!r.ok) throw new Error(p + ": " + r.status); return new Uint8Array(await r.arrayBuffer()); }
function push(b) { const p = wasm.wasm_alloc(b.length); new Uint8Array(mem.buffer, p, b.length).set(b); return p; }
function gate(root, trailer, ctx) {
  const r = push(root), t = push(trailer), c = push(ctx);
  const t0 = performance.now();
  const ok = wasm.verify(r, t, trailer.length, c, ctx.length);
  const ms = performance.now() - t0;
  wasm.wasm_free(r, root.length); wasm.wasm_free(t, trailer.length); wasm.wasm_free(c, ctx.length);
  return { ok: ok === 1, ms };
}

// Parse the trailer header far enough to expose the proof's shape. The layout
// is fixed by serialize_ext.rs: fp is 8 bytes, fp2 is 16, a Merkle path is a
// u32 count then that many 32-byte digests.
function anatomy(trailer) {
  const dv = new DataView(trailer.buffer, trailer.byteOffset, trailer.length);
  let o = 8;                                   // past the magic
  const depth = trailer[o]; o += 1;
  const siblings = [];
  for (let i = 0; i < depth; i++) { siblings.push(hex(trailer.slice(o, o + 32))); o += 32; }
  const dirBytes = Math.ceil(depth / 8);
  const dirs = [];
  for (let i = 0; i < depth; i++) dirs.push((trailer[o + (i >> 3)] >> (i & 7)) & 1);
  o += dirBytes;
  const traceRoot = hex(trailer.slice(o, o + 32)); o += 32;
  const compRoot = hex(trailer.slice(o, o + 32)); o += 32;
  const oodLen = dv.getUint32(o, true); o += 4; o += oodLen * 16;
  const friLayers = dv.getUint32(o, true); o += 4;
  const friRoots = [];
  for (let i = 0; i < friLayers; i++) { friRoots.push(hex(trailer.slice(o, o + 32))); o += 32; }
  const finalLen = dv.getUint32(o, true); o += 4; o += finalLen * 16;
  const friQueries = dv.getUint32(o, true);
  return { depth, siblings, dirs, traceRoot, compRoot, oodLen, friLayers, friRoots, finalLen, friQueries };
}

const unhex = h => new Uint8Array(h.match(/../g).map(x => parseInt(x, 16)));

// The context is rebuilt here from the values shown on screen, never fetched.
// What the proof binds is therefore exactly what the visitor reads: the
// measurement, the capability mask, the epoch. A served context could bind
// anything; a reconstructed one cannot.
function buildContext(it) {
  const meas = unhex(it.measurement);
  const be = n => { const b = new Uint8Array(8); let v = BigInt(n); for (let i = 7; i >= 0; i--) { b[i] = Number(v & 0xffn); v >>= 8n; } return b; };
  if (it.kind === "kernel") {
    const c = new Uint8Array(40); c.set(meas, 0); c.set(be(index.epoch), 32); return c;
  }
  const c = new Uint8Array(48); c.set(meas, 0); c.set(be(it.caps), 32); c.set(be(index.epoch), 40); return c;
}

function reservedLeaf() {
  const o = wasm.wasm_alloc(32);
  wasm.reserved_leaf(o);
  const r = new Uint8Array(mem.buffer, o, 32).slice();
  wasm.wasm_free(o, 32); return r;
}

function foldRoot(leaves) {
  const p = push(leaves), o = wasm.wasm_alloc(32);
  const t0 = performance.now();
  const ok = wasm.fold_root(p, leaves.length / 32, o);
  const ms = performance.now() - t0;
  const root = new Uint8Array(mem.buffer, o, 32).slice();
  wasm.wasm_free(p, leaves.length); wasm.wasm_free(o, 32);
  return { ok: ok === 1, root, ms };
}

// slot index from the trailer's direction bits: bit k of the index is the
// k-th direction, so the path IS the address
function slotOf(trailer) {
  const depth = trailer[8]; const off = 9 + depth * 32;
  let idx = 0;
  for (let i = 0; i < depth; i++) idx |= ((trailer[off + (i >> 3)] >> (i & 7)) & 1) << i;
  return idx;
}

function median(a) { if (!a.length) return 0; const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; }
function refresh() { $("s-ok").textContent = times.length; if (times.length) $("s-ms").textContent = median(times).toFixed(0) + " ms"; }

function detailHtml(it, a, meas, ms) {
  const path = a.siblings.map((s, i) =>
    `<div class="pl"><span class="lv">L${i}</span><span class="sib mono full">${s}</span><span class="dir">${a.dirs[i] ? "node is right child" : "node is left child"}</span></div>`
  ).join("");
  const roots = a.friRoots.map((r, i) => `<div class="fl"><span class="lv">fold ${i}</span><span class="mono full">${r}</span><span class="dom">domain / ${2 ** (i + 1)}</span></div>`).join("");
  return `<div class="detail">
    <div class="col">
      <div class="ch">Merkle authentication path &nbsp; slot ${slotOf(cache[it.slug].tr)} / 256 &nbsp; depth ${a.depth}</div>
      <div class="pl"><span class="lv">leaf</span><span class="sib mono full">${meas}</span><span class="dir">BLAKE3 of the binary, bound with capabilities and epoch</span></div>
      ${path}
      <div class="pl"><span class="lv">root</span><span class="sib dir">folds to the policy root in the header, or verification fails</span><span class="dir"></span></div>
    </div>
    <div class="col">
      <div class="ch">STARK proof anatomy</div>
      <div class="kv"><span>trace commitment</span><span class="mono full">${a.traceRoot}</span></div>
      <div class="kv"><span>composition commitment</span><span class="mono full">${a.compRoot}</span></div>
      <div class="kv"><span>out-of-domain frame</span><span class="mono">${a.oodLen} field elements</span></div>
      <div class="kv"><span>FRI folding layers</span><span class="mono">${a.friLayers}</span></div>
      ${roots}
      <div class="kv"><span>final layer</span><span class="mono">${a.finalLen} coefficients</span></div>
      <div class="kv"><span>FRI queries</span><span class="mono">${a.friQueries}</span></div>
      <div class="done">verified in ${ms.toFixed(0)} ms: recomputed the leaf, walked ${a.depth} Merkle levels to the root, checked ${a.friQueries} FRI queries for proximity to low degree over ${a.friLayers} folds, confirmed the grinding.</div>
    </div>
  </div>`;
}

async function verifyOne(it) {
  const root = it.kind === "kernel" ? kernelRoot : policyRoot;
  $("v-" + it.slug).className = "verdict run"; $("v-" + it.slug).textContent = "verifying";
  if (!cache[it.slug]) {
    cache[it.slug] = { tr: await bytes(it.slug + ".trailer.bin") };
  }
  const { tr } = cache[it.slug];
  const ctx = buildContext(it);
  const res = gate(root, tr, ctx);
  // the proof's Merkle directions are its slot address; a capsule must sit at
  // its enrollment position and the kernel at slot zero of its own tree
  const slot = slotOf(tr);
  const expect = it.kind === "kernel" ? 0 : index.items.filter(x => x.kind === "capsule").indexOf(it);
  if (res.ok && slot !== expect) { res.ok = false; res.misplaced = slot; }
  const a = anatomy(tr);
  const meas = hex(ctx.slice(0, 32));
  const det = $("d-" + it.slug);
  if (res.ok) {
    times.push(res.ms);
    $("v-" + it.slug).className = "verdict ok"; $("v-" + it.slug).textContent = "verified " + res.ms.toFixed(0) + " ms";
    det.innerHTML = detailHtml(it, a, meas, res.ms);
    det.style.display = "";
  } else {
    $("v-" + it.slug).className = "verdict bad";
    $("v-" + it.slug).textContent = res.misplaced !== undefined ? "WRONG SLOT " + res.misplaced : "REFUSED";
  }
  refresh();
  return res.ok;
}

function render() {
  const tb = $("rows"); tb.innerHTML = "";
  for (const it of index.items) {
    const caps = it.kind === "kernel" ? "<span class='cap'>boot chain</span>"
      : decodeCaps(it.caps).map(c => `<span class="cap">${c}</span>`).join("") || "<span class='cap'>none</span>";
    const tr = document.createElement("tr"); tr.className = "main";
    tr.innerHTML = `
      <td class="name">${it.slug}<span class="h">${it.handle}</span></td>
      <td><span class="kind ${it.kind}">${it.kind}</span></td>
      <td><div class="caps">${caps}</div></td>
      <td class="meas mono" title="${it.measurement}">${it.measurement.slice(0, 16)}…</td>
      <td class="mono dimc">${(it.trailer_bytes / 1024).toFixed(0)} KB</td>
      <td><span class="verdict idle" id="v-${it.slug}">not yet</span></td>
      <td><button class="rowbtn" data-s="${it.slug}">inspect</button></td>`;
    tb.appendChild(tr);
    const dr = document.createElement("tr"); dr.className = "drow";
    dr.innerHTML = `<td colspan="7"><div id="d-${it.slug}" style="display:none"></div></td>`;
    tb.appendChild(dr);
  }
  tb.querySelectorAll("button").forEach(b => { b.onclick = () => verifyOne(index.items.find(x => x.slug === b.dataset.s)); });
}

// ---- the boot chain, from the artifacts themselves -------------------------
function b3(bytes) {
  const p = push(bytes), o = wasm.wasm_alloc(32);
  wasm.blake3_hash(p, bytes.length, o);
  const d = new Uint8Array(mem.buffer, o, 32).slice();
  wasm.wasm_free(p, bytes.length); wasm.wasm_free(o, 32); return d;
}
function findOnce(hay, needle) {
  let hits = 0, at = -1;
  outer: for (let i = 0; i <= hay.length - needle.length; i++) {
    for (let j = 0; j < needle.length; j++) if (hay[i + j] !== needle[j]) continue outer;
    hits++; at = i; if (hits > 1) break;
  }
  return { hits, at };
}
function chainLine(cls, text) {
  const el = document.createElement("div"); el.className = "cl " + cls; el.textContent = text;
  $("chain-log").appendChild(el);
}
async function runChain(imgBytes, efiBytes) {
  $("chain-log").innerHTML = "";
  try {
    const dv = new DataView(imgBytes.buffer, imgBytes.byteOffset);
    const foot = imgBytes.length - 64;
    if (new TextDecoder().decode(imgBytes.slice(foot, foot + 8)) !== "NONOSIMG")
      return chainLine("bad", "not a NONOS image: footer magic missing");
    const kOff = dv.getUint32(foot + 24, true), kSize = dv.getUint32(foot + 28, true);
    const pOff = dv.getUint32(foot + 40, true), pSize = dv.getUint32(foot + 44, true);
    const rollback = dv.getUint32(foot + 56, true);
    const kernel = imgBytes.slice(kOff, kOff + kSize);
    const trailer = imgBytes.slice(pOff, pOff + pSize);
    chainLine("ok", `image footer parsed: kernel body ${kSize.toLocaleString()} bytes, embedded trailer ${pSize.toLocaleString()} bytes, rollback index ${rollback}`);
    if (new TextDecoder().decode(trailer.slice(0, 8)) !== "NZKSTRK1")
      return chainLine("bad", "embedded proof is not a STARK trailer");

    const boot = findOnce(efiBytes, kernelRoot);
    if (boot.hits !== 1) return chainLine("bad", `boot root found ${boot.hits} times in the bootloader; expected exactly one`);
    chainLine("ok", `boot root extracted from the bootloader binary, offset ${boot.at.toLocaleString()}, unique`);

    const t0 = performance.now();
    const meas = b3(kernel);
    chainLine("ok", `kernel measured in-wasm: BLAKE3 ${hex(meas).slice(0, 20)}… (${(kSize / 1048576).toFixed(0)} MiB in ${(performance.now() - t0).toFixed(0)} ms)`);

    const ctx = new Uint8Array(40); ctx.set(meas, 0); ctx[39] = 1;
    const res = gate(efiBytes.slice(boot.at, boot.at + 32), trailer, ctx);
    if (!res.ok) return chainLine("bad", "the STARK inside the image is REFUSED by the root inside the bootloader");
    chainLine("ok", `STARK from inside the image verified against the root from inside the bootloader, ${res.ms.toFixed(0)} ms`);

    const pol = findOnce(kernel, unhex(index.policy_root));
    if (pol.hits < 1) return chainLine("bad", "the capsule policy root is not embedded in this kernel");
    chainLine("ok", `capsule policy root found embedded in the kernel body, offset ${pol.at.toLocaleString()}: the chain continues to every capsule verified above`);
    chainLine("done", "boot chain closed: bootloader to kernel to policy root to capsules, every link from the artifacts themselves");
  } catch (e) { chainLine("bad", "chain failed: " + e.message); }
}

$("chain-default").onclick = async () => {
  $("chain-default").disabled = true;
  chainLine("run", "fetching the bundled shipping artifacts…");
  try {
    const [img, efi] = await Promise.all([bytes("kernel_attested.bin"), bytes("nonos_boot.efi")]);
    await runChain(img, efi);
  } catch (e) { chainLine("bad", e.message); }
  $("chain-default").disabled = false;
};
const dz = $("dropzone");
dz.ondragover = e => { e.preventDefault(); dz.classList.add("hot"); };
dz.ondragleave = () => dz.classList.remove("hot");
dz.ondrop = async e => {
  e.preventDefault(); dz.classList.remove("hot");
  const files = [...e.dataTransfer.files];
  let img = null, efi = null;
  for (const f of files) {
    const b = new Uint8Array(await f.arrayBuffer());
    if (b.length > 64 && new TextDecoder().decode(b.slice(b.length - 64, b.length - 56)) === "NONOSIMG") img = b;
    else if (b[0] === 0x4d && b[1] === 0x5a) efi = b;
  }
  if (!img) return chainLine("bad", "drop the kernel image (kernel_attested.bin) and optionally the bootloader (nonos_boot.efi)");
  if (!efi) { chainLine("run", "no bootloader dropped, using the bundled one"); efi = await bytes("nonos_boot.efi"); }
  await runChain(img, efi);
};

$("verify-all").onclick = async () => {
  $("verify-all").disabled = true; $("reset").disabled = true;
  times.length = 0; refresh();
  for (const it of index.items) await verifyOne(it);
  $("verify-all").disabled = false; $("reset").disabled = false;
};
$("reset").onclick = () => {
  times.length = 0; refresh();
  index.items.forEach(it => { $("v-" + it.slug).className = "verdict idle"; $("v-" + it.slug).textContent = "not yet"; $("d-" + it.slug).style.display = "none"; });
};

(async () => {
  try {
    const wasmBytes = await bytes("verifier.wasm");
    const { instance } = await WebAssembly.instantiate(wasmBytes, {});
    wasm = instance.exports; mem = wasm.memory;
    index = JSON.parse(new TextDecoder().decode(await bytes("index.json")));
    [policyRoot, kernelRoot] = await Promise.all([bytes("policy_root.bin"), bytes("kernel.root.bin")]);
    $("s-total").textContent = index.items.length;
    $("root").textContent = index.policy_root;
    $("epoch").textContent = index.epoch;
    try {
      const [leaves, kleaves] = await Promise.all([bytes("leaves.bin"), bytes("kernel_leaves.bin")]);
      const members = index.items.filter(x => x.kind === "capsule").length;
      const f = foldRoot(leaves), fk = foldRoot(kleaves);
      const match = f.ok && hex(f.root) === index.policy_root;
      const kmatch = fk.ok && hex(fk.root) === hex(kernelRoot);
      const pad = hex(reservedLeaf());
      let padOk = true;
      for (let i = members; i < 256; i++) if (hex(leaves.slice(i * 32, i * 32 + 32)) !== pad) padOk = false;
      for (let i = 1; i < 256; i++) if (hex(kleaves.slice(i * 32, i * 32 + 32)) !== pad) padOk = false;
      const el = $("recon");
      if (match && kmatch && padOk) {
        el.className = "recon ok";
        el.innerHTML = `set transparency: this page just refolded the complete tree, all 256 slots, ${members} members plus ${256 - members} reserved, through 255 Poseidon compressions in ${(f.ms + fk.ms).toFixed(0)} ms, and reproduced both roots exactly, and confirmed every one of the ${256 - members + 255} remaining slots is the reserved pad. Under these roots, these members exist and nothing else does.`;
      } else {
        el.className = "recon bad";
        el.textContent = padOk ? "ROOT RECONSTRUCTION FAILED: the leaf set does not fold to the enforced root" : "COMPLETENESS FAILED: a non-member slot is not the reserved pad";
      }
    } catch (e) { $("recon").textContent = "leaf set unavailable: " + e.message; }
    render();
    $("verify-all").disabled = false; $("reset").disabled = false;
    if (location.hash === "#auto") $("verify-all").click();
    const m = location.hash.match(/#one=(\w+)/); if (m) { const it = index.items.find(x => x.slug === m[1]); if (it) verifyOne(it); }
  } catch (e) { $("root").textContent = "failed to load: " + e.message; }
})();
