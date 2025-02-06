#![warn(missing_docs)]
/*!
BMFont font descriptor parsing library.

Manipulate, import and export [BMFont](http://www.angelcode.com/products/bmfont/)
files in text, binary, XML formats and more.

## Overview

This crate provides building, manipulation, import, and export functions for BMFont descriptor files.

The core data object is the [Font].
This struct holds, in its entirety, the data contained within a BMFont descriptor file.
When paired with the associated texture bitmap file/s, we have the information required to render the font in question.

Due to the numerous graphics backends and usage requirements, this crate does not attempt to offer a universal rendering solution.

This crate contains no unsafe code.
Also, unless specified by compilation switches, it doesn't pull in any external dependencies.

## Basic usage

The modules are organized around the core BMFont file formats:
- `text` : text format
- `binary` : binary format
- `json` : JSON format, requires: `--features json`
- `xml` : XML format, requires: `--features xml`

Each module is provides a number of import `from_...` and export: `to_...` functions.

To use:
1. Select the desired BMFont format you want to work with.
2. Select the appropriate from/ to methods based on the data structures you want to work with.

Example: import a BMFont text format file.

```no_run
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
 ```no_run
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

## Advanced usage - broken files

Unfortunately, there exist several BMFont tools that output broken files.
Either they do not comply with the BMFont standard as written or contain other errors.
When attempting to load these files, `bmfont_rs` will emit an error describing the problem.

We may be able to work around or ignore some of these problems using the [LoadSettings] struct.
Simply build the `LoadSettings` instance using the desired behavior switches and pass it into the `ext` form of the load function.

If you encounter a BMFont file that appears to work with other tools, but not `bmfont_rs` then kindly open a ticket.
It may be possible to add the correct behavior switch in future versions of `bmfont_rs`.

Example: import a BMFont text file with incorrect character counts.
```no_run
use std::io;
use std::io::prelude::*;
use std::fs;

fn main() -> bmfont_rs::Result<()> {
    let src = fs::read_to_string("font.txt")?;
    let settings = bmfont_rs::LoadSettings::default().ignore_counts();
    let font = bmfont_rs::text::from_str_ext(&src, &settings)?;
    println!("{:?}", font);
    Ok(())
}
```

## Advanced usage - string safety

This library defines unsafe strings as those containing ASCII control characters. Specifically ASCII codes 00 to 31 (inclusive) and 127.

When attempting to load files containing ASCII control characters, an [UnsafeValueString](crate::Error::UnsafeValueString) error is thrown. This behavior can be disabled using the [LoadSettings] struct.

Any additional string/ input sanitization MUST be undertaken by users in accordance with their use cases.

The BMFont format specifies strings at:
- [Info::face]
- [Font::pages]

## Rendering fonts

The [render.rs](https://github.com/shampoofactory/bmfont_rs/blob/main/examples/render.rs)
example, demonstrates a simple way to render font text to an image.
Substituting your own graphics backend should not be too difficult.

To view the example's output and for details on how to run it, kindly refer to the repository
[README](https://github.com/shampoofactory/bmfont_rs/blob/main/README.md#examples-render).

Due to the numerous graphics back-ends and usage requirements, this crate makes no attempt at
offering a universal rendering solution.

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

## Examples: JSON

JSON functionality is feature gated: `--features json`.
When activated, additional dependencies are pulled in assist with JSON processing.

Execute from the project root with:
```bash
cargo run --example json --features json
```

## Examples: XML

XML functionality is feature gated: `--features xml`.
When activated, additional dependencies are pulled in assist with XML processing.

Execute from the project root with:
```bash
cargo run --example xml --features xml
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
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
*/
mod builder;
mod charset;
mod error;
mod font;
mod parse;
mod settings;
mod tagged_attributes;

#[cfg(test)]
mod tests;

pub mod binary;
#[cfg(feature = "json")]
pub mod json;
pub mod text;
#[cfg(feature = "xml")]
pub mod xml;

pub use charset::*;
pub use error::{Error, Result};
pub use font::{Char, Chnl, Common, Font, Info, Kerning, Packing, Padding, Spacing};
pub use settings::LoadSettings;
