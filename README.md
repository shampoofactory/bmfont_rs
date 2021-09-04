![MIT/Apache 2.0 Licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
[![Rust](https://github.com/shampoofactory/bmfont_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/shampoofactory/bmfont_rs/actions)

BMFont font descriptor parsing library.

Manipulate, import and export [BMFont](http://www.angelcode.com/products/bmfont/) descriptor
files in text, binary, XML formats and more.

## TODO - 95% there

* crate documentation + proofread
* examples folder, notably a rendered text example
* check kcov coverage
* minimum Rust version
* upload to crates.io

## Overview

This crate provides manipulation, import and export functions for BMFont descriptor files.

The core data object is the [Font](crate::Font).
This object holds, in it's entirety, the information contained within a BMFont descriptor file.
When paired with the associated texture file/s, allows us to render the described font.

This crate contains no unsafe code and minimal dependencies.

## Examples

Load a BMFont text file.

```rust
use std::io;
use std::io::prelude::*;
use std::fs;

fn main() -> bmfont_rs::Result<()> {
    let mut buf = fs::read("font.txt")?;
    let font = bmfont_rs::text::from_bytes(&buf)?;
    println!("{:?}", font);
    Ok(())
}
```

Store a BMFont binary file.
 ```rust
 use std::io;
 use std::io::prelude::*;
 use std::fs::File;

 fn main() -> bmfont_rs::Result<()> {
     let font = bmfont_rs::Font::default();
     let mut writer = File::create("font.bin")?;
     bmfont_rs::binary::to_writer(&mut writer, &font)?;
     Ok(())
 }
 ```

TODO: Additional examples folder.


TODO: Example of how to render basic fonts.

## XML

XML support is featured gated with `xml` and pulls in additional dependencies,
namely [roxmltree](https://github.com/RazrFalcon/roxmltree).

TODO: cargo.toml entry and code example

## Additional formats

[Font](crate::Font) implements [Serde](https://serde.rs) `Serialize` and `Deserialize`.

TODO: Fill and describe JSON.

## BMFont

The BMFont homepage is [here](http://www.angelcode.com/products/bmfont/). The site includes
detailed [documentation](http://www.angelcode.com/products/bmfont/documentation.html), BMFont itself
and source code.

I am in no way affiliated with `www.angelcode.com` or BMFont.
All trademarks belong to their respective owners.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Alternatives

TODO
