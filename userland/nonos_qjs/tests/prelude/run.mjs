// NONOS Operating System (AGPL-3.0-or-later)
// Runs the shipped prelude and checks it behaves. It is five kilobytes of
// JavaScript that until now only ever ran inside a booted capsule, where a
// mistake in it shows up as a page that quietly does nothing.

import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { extractPrelude } from './extract.mjs';
import { classListChecks } from './classlist.mjs';
import { historyChecks } from './history.mjs';
import { lifecycleChecks, stubDocument } from './lifecycle.mjs';
import { navigatorChecks } from './navigator.mjs';
import { locationChecks } from './location.mjs';
import { computedChecks } from './computed.mjs';
import { datasetChecks } from './dataset.mjs';
import { dialogChecks } from './dialogs.mjs';
import { eventChecks } from './events.mjs';
import { scrollChecks } from './scroll.mjs';
import { storageChecks } from './storage.mjs';
import { timerChecks } from './timers.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const source = join(here, '..', '..', 'vendor', 'dom_bindings.c');
const requestSource = join(here, '..', '..', '..', 'capsule_browser', 'src', 'browser', 'http', 'request.rs');

// The prelude replaces console with a sink, which is right in a capsule and
// would silence these checks, so stdout is held onto first.
const say = s => process.stdout.write(`${s}\n`);

const prelude = extractPrelude(source);
// The capsule installs document before the prelude runs; so does this.
stubDocument();
// eslint-disable-next-line no-new-func
new Function(prelude)();

let failed = 0;
const ok = (cond, what) => {
  if (!cond) {
    say(`FAIL  ${what}`);
    failed += 1;
  }
};

let ran = 0;
const counting = (cond, what) => {
  ran += 1;
  ok(cond, what);
};

for (const [name, check] of [
  ['classList', classListChecks],
  ['computed', computedChecks],
  ['dataset', datasetChecks],
  ['dialogs', dialogChecks],
  ['events', eventChecks],
  ['timers', timerChecks],
  ['scroll', scrollChecks],
  ['storage', storageChecks],
  ['location', locationChecks],
  ['history', historyChecks],
  ['lifecycle', lifecycleChecks],
  ['navigator', ok => navigatorChecks(ok, requestSource)],
]) {
  try {
    check(counting);
  } catch (e) {
    say(`FAIL  ${name} threw: ${e && e.stack ? e.stack : e}`);
    failed += 1;
  }
}

if (failed > 0) {
  say(`${failed} of ${ran} prelude checks failed`);
  process.exit(1);
}
say(`${ran} prelude checks pass`);
