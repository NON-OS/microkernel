// NONOS Operating System (AGPL-3.0-or-later)
// The proof verifier's WebAssembly module, driven the one way both the page
// and the headless smoke test drive it: hand over the files by name, run the
// command line's arguments, read back its exit code and what it printed.

const enc = new TextEncoder();
const dec = new TextDecoder();

export async function load(wasmBytes) {
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  const x = instance.exports;
  const put = (bytes) => {
    if (bytes.length === 0) return [0, 0];
    const p = x.npv_alloc(bytes.length);
    if (!p) throw new Error("the verifier could not allocate " + bytes.length + " bytes");
    new Uint8Array(x.memory.buffer, p, bytes.length).set(bytes);
    return [p, bytes.length];
  };
  const free = ([p, n]) => { if (p) x.npv_free(p, n); };
  const take = (fn) => {
    const n = fn(0, 0);
    if (n === 0) return "";
    const b = put(new Uint8Array(n));
    try {
      fn(b[0], n);
      return dec.decode(new Uint8Array(x.memory.buffer, b[0], n).slice());
    } finally { free(b); }
  };
  return {
    /** files: Map of name to Uint8Array; args: the command line, no program name. */
    run(files, args) {
      x.npv_reset();
      for (const [name, bytes] of files) {
        const n = put(enc.encode(name)), d = put(bytes);
        try {
          if (x.npv_file(n[0], n[1], d[0], d[1]) !== 0) throw new Error("the verifier refused the file " + name);
        } finally { free(n); free(d); }
      }
      if (args.some((a) => a.includes("\0"))) throw new Error("an argument holds a NUL");
      const a = put(enc.encode(args.join("\0")));
      let code;
      try { code = x.npv_run(a[0], a[1]); } finally { free(a); }
      return { code, stdout: take(x.npv_stdout), stderr: take(x.npv_stderr) };
    },
  };
}
