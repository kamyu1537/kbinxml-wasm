# kbinxml-wasm
kbinxml-rs 를 node js 에서 쓸수 있도록 WebAssembly 로 빌드합니다.

> kbinxml-rs 의 cli 코드를 보고 작성하였습니다.<br/>
> 불필요한 코드가 들어갔을수도 있습니다.

## build
> rust 와 wasm-pack 이 필요합니다!<br/>
> https://www.rust-lang.org/
> https://rustwasm.github.io/wasm-pack/
> **만약 visual studio 가 없을경우 설치하셔야합니다!!!**
```shell script
npm run build
```

## how to use?
> 값이 `base64` 로 넘어옵니다.<br/>
> **해당부분은 따로 처리해주세요.**

``` javascript
const fs = require('fs');
const kbinxml = require('./kbinxml_wasm');

kbinxml.encode(fs.readFileSync('xml.xml')); // xml 을 binary 파일로 변환
kbinxml.decode(fs.readFileSync('kbin.kbin')); // binary 파일을 xml 로 변환
```
