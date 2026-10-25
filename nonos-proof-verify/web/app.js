// NONOS Operating System (AGPL-3.0-or-later)
// The page: the form becomes the command line's arguments and its files, the
// module runs them, and the v1 JSON report it prints is what is shown. The
// page decides nothing about a proof itself.

import { load } from "./npv.mjs";

const $ = (id) => document.getElementById(id);
let npv = null;

const fileOf = (id) => $(id).files[0] || null;
const bytesOf = async (f) => new Uint8Array(await f.arrayBuffer());

function showFor(mode) {
  for (const el of document.querySelectorAll("[data-for]")) {
    el.classList.toggle("hidden", !el.dataset.for.split(" ").includes(mode));
  }
}

/* Names the module sees: each file's own name, made unique, never starting
 * with "-" so no name reads as an option. */
function namer() {
  const used = new Set();
  return (f) => {
    let base = f.name.replace(/^-+/, "_") || "file";
    let name = base;
    for (let i = 2; used.has(name); i++) name = i + "-" + base;
    used.add(name);
    return name;
  };
}

async function request() {
  const mode = $("mode").value;
  const files = new Map();
  const name = namer();
  const add = async (f) => { const n = name(f); files.set(n, await bytesOf(f)); return n; };
  const args = [mode];
  if (mode === "transcript") {
    const t = fileOf("transcript");
    if (t) args.push(await add(t));
  }
  const root = fileOf("root");
  if (root) args.push("--root", await add(root));
  if (mode !== "transcript") {
    const img = fileOf("image");
    if (img) args.push("--image", await add(img));
  }
  if (mode === "capsule") {
    const caps = $("caps").value.trim();
    if (caps) args.push("--caps", caps);
  }
  if ($("dev").checked) args.push("--allow-dev");
  const trailers = [...$("trailers").files];
  if (trailers.length) args.push("--");
  for (const t of trailers) args.push(await add(t));
  return { files, args };
}

const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
const shellQuote = (s) => /^[\w@%+=:,./-]+$/.test(s) ? s : "'" + s.replace(/'/g, "'\\''") + "'";

function render(code, report, args, stderr) {
  const v = report.verdict;
  const n = report.results.length;
  const counts = (w) => report.results.filter((r) => r.verdict === w).length;
  const box = $("verdict");
  box.className = v;
  let head, sub;
  if (v === "verified") {
    head = `Verified: ${n} of ${n}`;
    sub = report.release ? "Under a release root, every trailer a STARK proof."
      : (report.root && !report.root.release) ? "Development root, not a release."
      : "Development trailers (no STARK), not a release.";
  } else if (v === "failed") {
    head = `Failed: ${counts("failed")} of ${n} refused`;
    sub = report.refusal || "A refused trailer is not enrolled under this root for what was given.";
  } else {
    head = "Could not check";
    sub = (report.refusal || stderr || `${counts("could-not-check")} of ${n} could not be checked`) + ". Nothing here is verified.";
  }
  box.innerHTML = `${esc(head)} <span class="kv">(exit ${code})</span><small>${esc(sub)}</small>`;
  const r = report.root;
  $("root-line").innerHTML = r
    ? `root ${esc(r.hex)}<br>${r.release ? "the release " + esc(r.name) + " (" + esc(r.file) + ")" : "development root, not a release"}`
      + (report.transcript ? `<br>transcript: ${report.transcript.slots} slots, root reproduced` : "")
    : "";
  $("rows").innerHTML = report.results.map((x) => {
    const what = x.verdict === "verified" ? (x.proof === "stark" ? "path and STARK" : "path only (development, no STARK)")
      : x.verdict === "failed" ? `${x.stage} [${x.code}]: ${x.reason}` : x.reason;
    const label = x.verdict === "could-not-check" ? "could not check" : x.verdict;
    const slot = x.slot === null ? "" : `${x.kind || ""} ${x.slot}`;
    return `<tr><td class="kv">${esc(x.trailer)}</td><td>${esc(slot)}</td><td class="v-${x.verdict}">${esc(label)}</td><td>${esc(what)}`
      + (x.caps ? `<br><span class="kv">caps ${esc(x.caps)}</span>` : "") + `</td></tr>`;
  }).join("");
  $("json").textContent = JSON.stringify(report, null, 1);
  $("cmd").textContent = ["nonos-proof-verify", ...args].map(shellQuote).join(" ");
  $("out").classList.remove("hidden");
}

async function verify() {
  $("go").disabled = true;
  // A result on screen is only ever this run's: the last one goes first.
  $("out").classList.add("hidden");
  $("verdict").className = "";
  $("verdict").textContent = "";
  $("rows").textContent = "";
  $("status").textContent = "checking…";
  try {
    const { files, args } = await request();
    const t0 = performance.now();
    // --json goes before any "--", past which every argument is a file.
    const out = npv.run(files, [args[0], "--json", ...args.slice(1)]);
    const ms = Math.round(performance.now() - t0);
    render(out.code, JSON.parse(out.stdout), args, out.stderr.trim());
    $("status").textContent = `${ms} ms`;
  } catch (e) {
    $("status").textContent = "error: " + e.message;
  } finally {
    $("go").disabled = false;
  }
}

$("mode").addEventListener("change", (e) => showFor(e.target.value));
$("go").addEventListener("click", verify);
$("show-json").addEventListener("click", () => $("json").classList.toggle("hidden"));
$("show-cmd").addEventListener("click", () => $("cmd").classList.toggle("hidden"));
showFor($("mode").value);

(async () => {
  try {
    const r = await fetch("nonos_proof_verify_wasm.wasm");
    if (!r.ok) throw new Error("nonos_proof_verify_wasm.wasm: " + r.status);
    npv = await load(new Uint8Array(await r.arrayBuffer()));
    $("go").disabled = false;
  } catch (e) {
    $("status").textContent = "the verifier did not load: " + e.message;
  }
})();
