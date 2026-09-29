// NONOS Operating System (AGPL-3.0-or-later)
// Pull the prelude out of the C source it ships in, so these checks run the
// text the engine actually evaluates rather than a copy that drifts from it.

import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const START = 'static const char *PRELUDE =';

/* The source as the compiler reads it: each #include "..." of a file next
 * to it replaced by that file's lines, so a prelude kept in pieces reads
 * as one. System headers (<...>) and files not beside it are left alone. */
function sourceLines(path) {
  return readFileSync(path, 'utf8').split('\n').flatMap(l => {
    const inc = l.trim().match(/^#include "([^"]+)"$/);
    const file = inc && join(dirname(path), inc[1]);
    return file && existsSync(file) ? sourceLines(file) : [l];
  });
}

export function extractPrelude(cPath) {
  const lines = sourceLines(cPath);
  const at = lines.findIndex(l => l.startsWith(START));
  if (at < 0) throw new Error(`no prelude in ${cPath}`);

  let out = '';
  for (const line of lines.slice(at + 1)) {
    const text = line.trim();
    // The definition ends at the statement's semicolon on its own.
    if (text === ';') return out;
    // Comments sit between the string pieces and are not part of the value.
    if (text.startsWith('/*') || text.startsWith('*') || text === '') continue;
    const piece = text.match(/^"(.*)"\s*;?$/);
    if (!piece) throw new Error(`unparsed prelude line: ${text}`);
    out += unescapeC(piece[1]);
    if (text.endsWith(';')) return out;
  }
  throw new Error('prelude never terminated');
}

// Only the escapes the prelude actually uses. Anything else is a mistake
// worth failing on rather than passing through and behaving differently in
// the engine than it does here.
function unescapeC(s) {
  let out = '';
  for (let i = 0; i < s.length; i++) {
    if (s[i] !== '\\') {
      out += s[i];
      continue;
    }
    const next = s[++i];
    if (next === '\\') out += '\\';
    else if (next === '"') out += '"';
    else if (next === 'n') out += '\n';
    else throw new Error(`unhandled escape \\${next}`);
  }
  return out;
}
