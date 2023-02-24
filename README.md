# kbinxml-wasm

rust kbinxml built with wasm to be usable in nodejs.

## Installation

```shell
npm i @kamyu/kbinxml
```

## Build

### Requirements

- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [rust](https://www.rust-lang.org/)

### Command

```shell
npm run build
```

### Type

```typescript
enum EncodingType {
    None = 0x00,
    ASCII = 0x20,
    ISO_8859_1 = 0x40,
    EUC_JP = 0x60,
    SHIFT_JIS = 0x80,
    UTF_8 = 0xA0,
}

type XmlResult = { data: string, encoding: EncodingType };
type BinaryResult = { data: Uint8Array, encoding: EncodingType };
type BinaryOptions = { encoding?: EncodingType, compression?: boolean };

to_bin(xml: string): BinaryResult;
to_bin_with_options(xml: string, options: BinaryOptions): BinaryResult;
to_xml(bin: Uint8Array): XmlResult;
```
