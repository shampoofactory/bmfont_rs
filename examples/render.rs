use bmfont_rs::{Char, Common, Font, Packing};
use image::{self, GrayImage, ImageFormat};

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::result::Result;

const FONT_DIR: &str = "data/examples";
const FONT: &str = "anton_latin.fnt";

const SURFACE_WIDTH: i32 = 600;
const SURFACE_HEIGHT: i32 = 300;

/// Basic rectangle.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rec2 {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Rec2 {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self { top_left, bottom_right }
    }

    pub fn with_size(top_left: Vec2, size: Vec2) -> Self {
        Self::new(top_left, Vec2::new(top_left.x + size.x, top_left.y + size.y))
    }
}

/// Basic length 2 vector
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A basic surface to render our font to.
pub struct RenderSurface {
    /// Render target
    dst: GrayImage,
    /// New font position
    pos: Vec2,
    /// Last character
    last: Option<char>,
}

impl RenderSurface {
    pub fn new(res: Vec2) -> Self {
        Self { dst: GrayImage::new(res.x as u32, res.y as u32), pos: Vec2::default(), last: None }
    }

    /// Save our font. Selects formats according to the path extension (png, jpg only).
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        self.dst.save(path)?;
        Ok(())
    }

    /// Print and newline. Text wrapping not implemented.
    pub fn println(&mut self, render_font: &RenderFont, str: &str) {
        self.print(render_font, str);
        // Newline.
        self.pos.x = 0;
        self.pos.y += render_font.common.line_height as i32;
        self.last = None;
    }

    /// Print. Text wrapping not implemented.
    pub fn print(&mut self, render_font: &RenderFont, str: &str) {
        str.chars().for_each(|character| self.print_character(render_font, character))
    }

    /// Print character. Text wrapping not implemented.
    pub fn print_character(&mut self, render_font: &RenderFont, character: char) {
        if let Some(char) = render_font.chars.get(&(character as u32)) {
            // Calculate the source image coordinates.
            let src_rect = Rec2::with_size(
                Vec2::new(char.x as i32, char.y as i32),
                Vec2::new(char.width as i32, char.height as i32),
            );

            // Calculate the destination image coordinates.
            // We aren't implementing text wrapping, but here would be the place to do it.
            let dst_pos =
                Vec2::new(self.pos.x + char.xoffset as i32, self.pos.y + char.yoffset as i32);

            // Advance our pos.
            self.pos.x += char.xadvance as i32;

            // Kerning pair adjustment for pos.
            if let Some(last) = self.last {
                let kerning_pair = (last as u32, character as u32);
                if let Some(amount) = render_font.kernings.get(&kerning_pair) {
                    self.pos.x += *amount as i32;
                }
            }
            self.last = Some(character);

            // Grab the correct bitmap page.
            let src = &render_font.bitmaps[char.page as usize];

            // Render.
            render(src, src_rect, &mut self.dst, dst_pos);
        } else {
            // Implement our missing character strategy.
            eprintln!("cannot render character: {:08X}", character as u32);
        }
    }
}

/// The Font data we need in an accessible format.
/// Chars and Kernings have been restructured as maps.
/// Unused items have been discarded.
pub struct RenderFont {
    /// Common field
    common: Common,
    /// Bitmaps
    bitmaps: Vec<GrayImage>,
    /// Characters keyed to u32 character
    chars: HashMap<u32, Char>,
    /// Kerning amount keyed to (u32 first character, u32 second character)
    kernings: HashMap<(u32, u32), i16>,
}

impl RenderFont {
    pub fn new(font: Font, bitmaps: Vec<GrayImage>) -> Result<Self, Box<dyn Error>> {
        // Check we don't have references to things that don't exist.
        font.validate_references()?;

        // Take what we need.
        let Font { common, mut chars, mut kernings, .. } = font;

        // Restructure Chars and Kernings into maps for efficiency.
        let chars = chars.drain(..).map(|u| (u.id, u)).collect();
        let kernings = kernings.drain(..).map(|u| ((u.first, u.second), u.amount)).collect();

        Ok(Self { common, bitmaps, chars, kernings })
    }
}

/// Load the bitmap font.
fn load_bitmap_font(
    folder: impl AsRef<Path>,
    font: impl AsRef<Path>,
) -> Result<(Font, Vec<GrayImage>), Box<dyn Error>> {
    let folder: &Path = folder.as_ref();
    let font: &Path = font.as_ref();

    // Load the font descriptor.
    let rdr = File::open(folder.join(font))?;
    let font = bmfont_rs::text::from_reader(rdr)?;

    // Manage info and common attributes.
    //
    // If you trust that the font descriptor file has been generated with the correct parameters,
    // you could skip this step.
    //
    // We are only supporting Unicode and 8-bit gray scale:
    //   info: unicode=1
    //   common: packed=0 alphaChnl=1
    if !font.info.unicode || font.common.packed || font.common.alpha_chnl != Packing::Outline {
        return Err(
            format!("unsupported font descriptor: {:?}, {:?}", font.info, font.common).into()
        );
    }

    // Load the textures
    let mut bitmaps = Vec::with_capacity(font.pages.len());
    for page in &font.pages {
        let rdr = BufReader::new(File::open(folder.join(page))?);
        let bitmap = image::load(rdr, ImageFormat::Png).map(|u| u.into_luma8())?;
        bitmaps.push(bitmap);
    }

    // Done! We have what we need to render the font.
    Ok((font, bitmaps))
}

/// Render from src to dst using the supplied dimensions. This function is inefficient.
/// In practice you likely want to render using a graphics capable API such as SDL, OpenGL or
/// similar.
fn render(src: &GrayImage, src_rect: Rec2, dst: &mut GrayImage, dst_pos: Vec2) {
    // Clamp height/ width to available src/ dst image dimensions.
    let src_pos = src_rect.top_left;
    let src_width = src_rect.bottom_right.x - src_pos.x;
    let src_height = src_rect.bottom_right.y - src_pos.y;
    let dst_width = dst.width() as i32 - dst_pos.x;
    let dst_height = dst.height() as i32 - dst_pos.y;
    let width = src_width.min(dst_width);
    let height = src_height.min(dst_height);
    // Copy over our pixels, one by one, slowly...
    for x in 0..width {
        for y in 0..height {
            let pixel = src.get_pixel((src_pos.x + x) as u32, (src_pos.y + y) as u32);
            dst.put_pixel((dst_pos.x + x) as u32, (dst_pos.y + y) as u32, *pixel);
        }
    }
}

/// Render basic text to an image file
///
/// Execute from the project root with:
/// ```
/// cargo run --example render FILE
/// ```
///
/// Where FILE is the output image destination with either a .png or .jpg extension.
///
/// Example:
/// ```
/// cargo run --example render ~/Desktop/lorem.png
/// ```

fn main() -> Result<(), Box<dyn Error>> {
    // Get output file from command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("missing image output file: png or jpg");
    }
    let file = &args[1];

    // Load our font and bitmaps.
    let (font, bitmaps) = load_bitmap_font(FONT_DIR, FONT)?;

    // We'll opt for an immutable RenderFont object that we can inject into render calls.
    let render_font = RenderFont::new(font, bitmaps)?;

    // Basic render surface.
    let mut render_surface = RenderSurface::new(Vec2::new(SURFACE_WIDTH, SURFACE_HEIGHT));

    // Ok! Let's print something.
    render_surface.println(&render_font, "Lorem ipsum dolor sit amet,");
    render_surface.println(&render_font, "consectetur adipiscing elit,");
    render_surface.println(&render_font, "sed do eiusmod tempor incididunt");
    render_surface.println(&render_font, "ut labore et dolore magna aliqua.");

    // Let's save and make a run for it.
    render_surface.save(file)?;

    // All done! Time for coffee and cookies...
    Ok(())
}
