# yew-counter-sample
Rust Yew Framework example with stack based on dev.to Tutorial hunt-the-wumpus.

## Agenda of the accompanied presentation

## 1. Definition

The WebAssembly object https://developer.mozilla.org/en-US/docs/WebAssembly
- Intermediate „Code format“
- Any language with compiler build target wasm
- Readable WAT, Binary WASM
- Stack Machine (LIFO), Byte Array

Webassembly Languages: https://github.com/appcypher/awesome-wasm-langs

## 2. Examples

- Counter (JS binding)
https://webassembly.studio/?f=ap6yke0bmw

- Bonus example: Counter (Full WASM Web Application)

__This repository__

- Super extra bonus example: Blazor with Material Design
https://blazorcomponents.github.io/MatBlazor/MatTextField

## 3. Browser Availability / Polyfill

- Browser Availability https://caniuse.com/#search=webassembly

- It took only two years for all browser vendors to get on the same page […], and as of October 2017, all major browsers support it. https://www.bleepingcomputer.com/news/security/all-major-browsers-now-support-webassembly/

What are the limitations?
- No garbage collection (planned, language specific)
- No direct DOM access (language specific APIs via JavaScript, if available)
- No support in older browsers
    - Browser „Polyfilling“ with Emscripten  https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Emscripten
    - But IE11 Support is receiving not much attention: https://github.com/emscripten-core/emscripten/issues/6204


=> „Welcome! Bring your own chair.“

## 4. Performance comparison via js-frameworks-benchmark 
https://stefankrause.net/js-frameworks-benchmark8/table.html


