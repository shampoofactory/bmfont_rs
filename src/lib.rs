#![warn(missing_docs)]
/*!
BMFont parsing library.

Manipulate, import and export [BMFont](http://www.angelcode.com/products/bmfont/)
files in text, binary, XML formats and more.

## Overview

This crate provides manipulation, import and export functions for BMFont descriptor files.

The core data object is the [Font](crate::Font).
This object holds, in it's entirety, the information contained within a BMFont descriptor file.
Font, when paired with the associated texture file/s, allows us to render the described bit-mapped
text.

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

## Rendering fonts

The [render.rs](https://github.com/shampoofactory/bmfont_rs/blob/main/examples/render.rs)
example, demonstrates a simple way to render font text to an image.
Substituting your own graphics backend should not be too difficult.

To view the example's output and for details on how to run it, kindly refer to the repository
[README](https://github.com/shampoofactory/bmfont_rs/blob/main/README.md#examples-render).

Due to the numerous graphics back-ends and usage requirements, this crate makes no attempt at
offering a universal rendering solution.

## Serde

[Font] implements [Serde's](https://github.com/serde-rs/serde) `serialize` and `deserialize` traits.
These are feature gated and require: `--features serde`.

## JSON

The [json.rs](https://github.com/shampoofactory/bmfont_rs/blob/main/examples/json.rs) example
demonstrates this.

For details on how to run the example, kindly refer to the repository
[README](https://github.com/shampoofactory/bmfont_rs/blob/main/README.md#examples-json).

JSON is not natively supported.
However, as we do support [Serde](https://github.com/serde-rs/serde), we can easily cobble together
support with [Serde JSON](https://github.com/serde-rs/serde).

By default our Serde serializers map boolean types to JSON boolean types: `true` and `false`.
However, at least one JSON BMFont parser expects integer boolean types: `1` and `0`.
To facilitate the latter we can pass `--features serde_boolint`, which casts boolean values to
integers and vice versa.

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
pub mod text;
#[cfg(feature = "xml")]
pub mod xml;

pub use charset::*;
pub use error::{Error, Result};
pub use font::{Char, Chnl, Common, Font, Info, Kerning, Packing, Padding, Spacing};
pub use settings::LoadSettings;
