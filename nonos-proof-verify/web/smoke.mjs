// NONOS Operating System (AGPL-3.0-or-later)
// Headless check of the page's verifier: the module, driven through the
// page's own loader, answers every case with the expected exit code, and
// prints byte for byte what the command line prints for the same arguments
// and files.
//
//   node web/smoke.mjs <nonos_proof_verify_wasm.wasm> [<nonos-proof-verify binary>]

import { readFileSync, readdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { load } from "./npv.mjs";

const [wasmPath, cliArg] = process.argv.slice(2);
const cliPath = cliArg && resolve(cliArg);
if (!wasmPath) {
  console.error("usage: node web/smoke.mjs <wasm> [<cli>]");
  process.exit(2);
}
const here = dirname(fileURLToPath(import.meta.url));
const crate = join(here, "..");
const top = join(crate, "..");
const npv = await load(readFileSync(wasmPath));

let failures = 0;
const check = (ok, what) => {
  console.log((ok ? "ok    " : "FAIL  ") + what);
  if (!ok) failures++;
};

/* A case: a directory the files live in, the files' names there, and the
 * arguments naming them. Some names may be left out on purpose. */
function run(dir, names, args, mutate = {}) {
  const files = new Map();
  for (const n of names) files.set(n, mutate[n] || new Uint8Array(readFileSync(join(dir, n))));
  return npv.run(files, args);
}

function cli(dir, args) {
  try {
    return { code: 0, stdout: execFileSync(cliPath, args, { cwd: dir, encoding: "utf8" }) };
  } catch (e) {
    return { code: e.status, stdout: e.stdout };
  }
}

function same(dir, names, args, want, what) {
  const out = run(dir, names, args);
  check(out.code === want, `${what}: exit ${out.code}, want ${want}`);
  const report = JSON.parse(out.stdout);
  check(report.exit_code === out.code && report.format === "nonos-proof-verify" && report.version === 1, `${what}: a v1 report`);
  if (want !== 0) check(report.verdict !== "verified", `${what}: not read as verified`);
  if (cliPath) {
    const c = cli(dir, args);
    check(c.code === out.code && c.stdout === out.stdout, `${what}: the command line prints the same`);
  }
  return report;
}

const fx = join(crate, "tests", "fixture");
const policy = join(top, "nonos-data", "trust", "policy");
const capsules = join(top, "nonos-data", "trust", "capsules");

// The release: every committed capsule trailer under the committed transcript.
{
  const trailers = readdirSync(capsules).filter((n) => n.endsWith(".zk_trailer.bin")).sort();
  const files = new Map([["zk_capsule_policy_root.bin.transcript", new Uint8Array(readFileSync(join(policy, "zk_capsule_policy_root.bin.transcript")))]]);
  for (const t of trailers) files.set(t, new Uint8Array(readFileSync(join(capsules, t))));
  const t0 = performance.now();
  const out = npv.run(files, ["transcript", "zk_capsule_policy_root.bin.transcript", ...trailers, "--json"]);
  const ms = Math.round(performance.now() - t0);
  const r = JSON.parse(out.stdout);
  check(out.code === 0 && r.release === true && r.results.length === trailers.length
    && r.results.every((x) => x.proof === "stark"), `release: ${trailers.length} committed capsule trailers verify under the release root (${ms} ms)`);
}

const capsule = (caps, trailer) => ["capsule", "--root", "caps_root.bin", "--image", "a.elf", "--caps", caps, trailer, "--json"];
const names = ["caps_root.bin", "a.elf", "a.trailer", "b.trailer", "b.elf"];
same(fx, names, capsule("0x18", "a.trailer"), 0, "capsule A");
same(fx, names, capsule("0x19", "a.trailer"), 1, "a changed capability bit");
same(fx, names, capsule("0x18", "b.trailer"), 1, "another capsule's trailer");
same(fx, names, ["capsule", "--root", "caps_root.bin", "--image", "b.elf", "--caps", "0x18", "a.trailer", "--json"], 1, "a swapped image");
same(fx, ["kernel_root.bin", "kernel.image", "kernel.trailer"], ["kernel", "--root", "kernel_root.bin", "--image", "kernel.image", "kernel.trailer", "--json"], 0, "kernel");
same(fx, ["caps_root.bin.transcript", "a.trailer", "b.trailer"], ["transcript", "caps_root.bin.transcript", "a.trailer", "b.trailer", "--json"], 0, "transcript");
same(fx, ["dev_root.bin", "a.elf", "a.dev"], ["capsule", "--root", "dev_root.bin", "--image", "a.elf", "--caps", "18", "a.dev", "--json"], 1, "a development trailer, not allowed");
const dev = same(fx, ["dev_root.bin", "a.elf", "a.dev"], ["capsule", "--root", "dev_root.bin", "--image", "a.elf", "--caps", "18", "--allow-dev", "a.dev", "--json"], 0, "a development trailer, allowed");
check(dev.release === false && dev.results[0].proof === "path-only", "a development trailer is never a release");
const boot = join(top, "nonos-boot-measure", "src", "tests", "fixture");
same(boot, ["root.bin", "loader.efi", "trailer.bin"], ["bootloader", "--root", "root.bin", "--image", "loader.efi", "trailer.bin", "--json"], 0, "bootloader");

// What the page was not given cannot be checked, and says so.
const missing = run(fx, ["caps_root.bin", "a.elf"], capsule("0x18", "a.trailer"));
check(missing.code === 2 && JSON.parse(missing.stdout).verdict === "could-not-check", "a trailer not handed over: could not check");
const usage = npv.run(new Map(), ["--json"]);
check(usage.code === 2 && usage.stderr.includes("unknown command"), "a usage error: could not check");

// Flipped bytes across the trailer, through the page's path.
{
  const honest = new Uint8Array(readFileSync(join(fx, "a.trailer")));
  let refused = 0;
  const at = [0, 8, 9, 13, 100, 278, 279, 283, 300, 1000, 50000, honest.length - 1];
  for (const i of at) {
    const t = honest.slice();
    t[i] ^= 0x01;
    const out = run(fx, names, capsule("0x18", "a.trailer"), { "a.trailer": t });
    if (out.code === 1) refused++;
  }
  check(refused === at.length, `${refused} of ${at.length} flipped trailers refused`);
}

console.log(failures ? `${failures} FAILED` : "all passed");
process.exit(failures ? 1 : 0);
