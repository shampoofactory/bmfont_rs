![MIT/Apache 2.0 Licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
[![Rust](https://github.com/shampoofactory/bmfont_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/shampoofactory/bmfont_rs/actions)

BMFont font descriptor parsing library.

Manipulate, import and export [BMFont](http://www.angelcode.com/products/bmfont/) descriptor
files in text, binary, XML formats and more.

## Overview

This crate provides manipulation, import and export functions for BMFont descriptor files.

The core data object is the [Font](crate::Font).
This object holds, in it's entirety, the information contained within a BMFont descriptor file.
Font, when paired with the associated texture file/s, allows us to render the described bit-mapped text.

This crate contains no unsafe code and minimal dependencies.

## Basic usage

The modules are organized around the core BMFont file formats:
- `text` : text format
- `binary` : binary format
- `xml` : XML format, requires: `--features xml`

Each module is provides a number of import `from_...` and export: `to_...` functions.

To use:
1. Select the desired BMFont format you want to work with.
2. Select the appropriate from/ to methods based on the data structures you want to work with.

Example: import a BMFont text format file.

```rust
use std::io;
use std::io::prelude::*;
use std::fs;

fn main() -> bmfont_rs::Result<()> {
    let mut buf = fs::read("font.fnt")?;
    let font = bmfont_rs::text::from_bytes(&buf)?;
    println!("{:?}", font);
    Ok(())
}
```

Example: export a BMFont text format file.
 ```rust
 use std::io;
 use std::io::prelude::*;
 use std::fs::File;

 fn main() -> bmfont_rs::Result<()> {
     let font = bmfont_rs::Font::default();
     let mut writer = File::create("font.fnt")?;
     bmfont_rs::text::to_writer(&mut writer, &font)?;
     Ok(())
 }
 ```

Additonal format and source/ destination parameters are supported.
Kindly refer to the documentation for details.

## Examples: render

![Alt text](data/examples/render_out.png)

The above text was generated with the [render.rs](examples/render.rs) example.

If you are uncertain how one might use a BMFont descriptor to render output, this example would be worth studying.
Substituting your own graphics backend should not be too difficult.


Due to the numerous graphics backends and usage requirements, this crate makes no attempt at offering a universal rendering solution.

Execute from the project root with:
```bash
cargo run --example render FILE
```

Where FILE is the output image destination (png or jpg) extension:

```bash
cargo run --example render ~/Desktop/lorem.png
```

## Examples: text format

BMFont text format files are ubiquitous, human readable and easily tinkered with.
However, not all tools obey the correct parameter types or constraints, which may result in incompatibility.

Execute from the project root with:
```bash
cargo run --example text
```

## Examples: binary

BMFont binary files are compact, unambiguous and efficient to parse.
However, tooling support may be limited and they are not human readable.

Execute from the project root with:
```bash
cargo run --example binary
```

## Examples: XML

XML functionality is feature gated: `--features xml`.
When activated, additional dependencies are pulled in assist with XML processing.

Execute from the project root with:
```bash
cargo run --example xml --features xml
```

## Examples: JSON

JSON is not natively supported.
However, as we do support [Serde](https://github.com/serde-rs/serde), we can easily cobble together support with [Serde JSON](https://github.com/serde-rs/serde).

By default our Serde serializers map boolean types to JSON boolean types: `true` and `false`.
However, at least one JSON BMFont parser expects integer boolean types: `1` and `0`.
To facilitate the latter we can pass `--features serde_boolint`, which casts boolean values to integers and vice versa.

Execute from the project root with:
```bash
cargo run --example json --features serde`
```

```bash
cargo run --example json --features "serde, serde_boolint"`
```

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
