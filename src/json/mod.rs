#[cfg(test)]
mod tests {
    use crate::charset::Charset;
    use crate::font::*;

    use std::error::Error;
    use std::result::Result;

    fn font() -> Font {
        let info = Info {
            face: "Nexa Light".to_owned(),
            size: 32,
            bold: false,
            italic: false,
            charset: Charset::Null,
            unicode: true,
            stretch_h: 100,
            smooth: true,
            aa: 2,
            padding: Padding { up: 0, right: 0, down: 0, left: 0 },
            spacing: Spacing { horizontal: 0, vertical: 0 },
            outline: 0,
        };
        let common = Common {
            line_height: 32,
            base: 24,
            scale_w: 1024,
            scale_h: 2048,
            pages: 1,
            packed: false,
            alpha_chnl: Packing::Glyph,
            red_chnl: Packing::Glyph,
            green_chnl: Packing::Glyph,
            blue_chnl: Packing::Glyph,
        };
        let pages = vec!["sheet.png".to_owned()];
        let chars = vec![
            Char {
                id: 10,
                x: 281,
                y: 9,
                width: 0,
                height: 0,
                xoffset: 0,
                yoffset: 24,
                xadvance: 8,
                page: 0,
                chnl: Chnl::NONE,
            },
            Char {
                id: 32,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                xoffset: 0,
                yoffset: 0,
                xadvance: 9,
                page: 0,
                chnl: Chnl::NONE,
            },
        ];
        let kernings = vec![
            Kerning { first: 34, second: 65, amount: -2 },
            Kerning { first: 34, second: 67, amount: 1 },
        ];
        Font { info, common, pages, chars, kernings }
    }

    #[test]
    fn json_load() -> Result<(), Box<dyn Error>> {
        let font = font();
        let bytes = include_bytes!("../../data/fnt.json");
        assert_eq!(serde_json::from_slice::<Font>(bytes)?, font);
        Ok(())
    }

    #[test]
    fn json_store() -> Result<(), Box<dyn Error>> {
        let font = font();
        let string = serde_json::to_string_pretty(&font)?;
        assert_eq!(serde_json::from_str::<Font>(&string)?, font);
        Ok(())
    }
}
