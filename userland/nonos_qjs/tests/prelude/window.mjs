// NONOS Operating System (AGPL-3.0-or-later)
// innerWidth, innerHeight, devicePixelRatio and screen were not defined, so
// a page sizing a canvas from them read undefined. Each now asks the host
// on every read, so a resize is seen without the page being reloaded.

export function windowChecks(ok) {
  const m = [683, 384, 5];
  globalThis.__njs_window = w => m[w] ?? 0;

  ok(globalThis.innerWidth === 683 && globalThis.outerWidth === 683, 'width is the viewport');
  ok(globalThis.innerHeight === 384 && globalThis.outerHeight === 384, 'height is the viewport');
  ok(globalThis.devicePixelRatio === 1.25, `five quarters is 1.25: ${globalThis.devicePixelRatio}`);
  const s = globalThis.screen;
  ok(s.width === 683 && s.availWidth === 683 && s.height === 384 && s.availHeight === 384,
    'screen is the viewport');
  ok(s.colorDepth === 24 && s.pixelDepth === 24, 'screen colour depth');

  m[0] = 1280; m[1] = 720; m[2] = 8;
  ok(globalThis.innerWidth === 1280 && globalThis.innerHeight === 720, 'a resize is read afresh');
  ok(globalThis.devicePixelRatio === 2 && s.width === 1280, 'so are the scale and screen');
}
