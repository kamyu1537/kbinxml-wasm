# kbinxml-wasm
rust kbinxml built with wasm to be usable in nodejs.

## Installation
```shell
npm i @kamyu/kbinxml
```

## Build
### Requirements
* [wasm-pack](https://rustwasm.github.io/wasm-pack/)
* [rust](https://www.rust-lang.org/)
### Command
```shell
npm run build
```

## Example
> **parameter**: Buffer or Uint8Array  
> **return**: String(Base64)

### Javascript
```javascript
const fs = require('fs');
const kbinxml = require('@kamyu/kbinxml');

kbinxml.encode(fs.readFileSync('xml.xml'));
kbinxml.decode(fs.readFileSync('xml.scripts'));
```
