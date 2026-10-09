// NONOS Operating System (AGPL-3.0-or-later)
// The page's life as scripts see it: readyState, DOMContentLoaded and load,
// window's own listeners, and document.cookie through the host. window had
// no addEventListener, so window.addEventListener('load', ...) threw; and
// nothing ever fired DOMContentLoaded, so code waiting on it never ran.

// What run.mjs installs before the prelude: a document whose native
// listener table records what reaches it, and the two cookie natives
// standing in for the host's jar.
export const natives = [];
export const jar = { value: '', writes: [] };

export function stubDocument() {
  // Node has a read-only navigator of its own; the capsule has none, and
  // the prelude's assignment has to land as it does there.
  Object.defineProperty(globalThis, 'navigator', {
    value: undefined, writable: true, configurable: true,
  });
  globalThis.document = {
    addEventListener(t, f) { natives.push([t, f]); },
    removeEventListener() {},
  };
  globalThis.__njs_cookie_get = () => jar.value;
  globalThis.__njs_cookie_set = s => { jar.writes.push(s); };
}

export function lifecycleChecks(ok) {
  const d = globalThis.document;
  const seen = [];
  ok(d.readyState === 'loading', `scripts run while loading: ${d.readyState}`);

  d.addEventListener('DOMContentLoaded', e => seen.push(`dcl:${d.readyState}:${e.type}`));
  d.addEventListener('readystatechange', () => seen.push(`rs:${d.readyState}`));
  globalThis.addEventListener('DOMContentLoaded', () => seen.push('win-dcl'));
  globalThis.addEventListener('load', () => seen.push(`load:${d.readyState}`));
  globalThis.addEventListener('load', () => seen.push('once'), { once: true });
  globalThis.onload = () => seen.push('onload');
  const gone = () => seen.push('removed');
  globalThis.addEventListener('load', gone);
  globalThis.removeEventListener('load', gone);
  globalThis.addEventListener('load', { handleEvent: () => seen.push('object') });
  globalThis.addEventListener('load', () => { throw new Error('a broken listener'); });
  globalThis.addEventListener('load', () => seen.push('after-throw'));

  d.addEventListener('click', () => {});
  ok(natives.length === 1 && natives[0][0] === 'click', 'other types reach the native table');

  ok(globalThis.__njs_ready('complete') === 0, 'complete cannot come before interactive');
  ok(globalThis.__njs_ready('interactive') === 1, 'interactive is reached once');
  ok(d.readyState === 'interactive', 'and DOMContentLoaded sees interactive');
  ok(globalThis.__njs_ready('interactive') === 0, 'and only once');
  ok(globalThis.__njs_ready('complete') === 1, 'complete follows');
  ok(d.readyState === 'complete', 'readyState ends complete');
  ok(globalThis.__njs_ready('complete') === 0, 'load fires once');

  const want = [
    'rs:interactive', 'dcl:interactive:DOMContentLoaded', 'win-dcl',
    'rs:complete', 'load:complete', 'once', 'object', 'after-throw', 'onload',
  ].join(',');
  ok(seen.join(',') === want, `the order a page sees: ${seen.join(',')}`);

  // A listener that throws is recorded and does not stop the ones after it.
  const errs = globalThis.__njs_errors || [];
  ok(errs.some(e => e.includes('a broken listener')), 'a throwing listener is recorded');

  const ev = new globalThis.Event('custom');
  let got = null;
  globalThis.addEventListener('custom', e => { got = e.target; });
  globalThis.dispatchEvent(ev);
  ok(got === globalThis, 'window.dispatchEvent reaches its own listeners');

  jar.value = 'a=1; b=2';
  ok(d.cookie === 'a=1; b=2', 'document.cookie reads the host jar');
  d.cookie = 'c=3; path=/';
  ok(jar.writes.length === 1 && jar.writes[0] === 'c=3; path=/', 'a write goes to the host as written');
}
