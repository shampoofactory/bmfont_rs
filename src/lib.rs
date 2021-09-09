#![warn(missing_docs)]
/*!
BMFont parsing library.

Manipulate, import and export [BMFont](http://www.angelcode.com/products/bmfont/)
files in text, binary, XML formats and more.

## Overview

This crate provides import and export functions for BMFont descriptor files.


The core data object is the [Font](crate::Font).
This object holds, in it's entirety, the information contained within a BMFont descriptor file.
This, when paired with the associated texture file/s, allows us to render the described font.

## Examples

Load a BMFont text file.

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

Store a BMFont text file.
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

TODO: Additional examples folder.
TODO: Example of how to render basic fonts.

## XML

XML support is featured gated with `xml` and pulls in additional dependencies,
namely [roxmltree](https://github.com/RazrFalcon/roxmltree).

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

*/
mod builder;
mod charset;
mod error;
mod font;
mod parse;
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
