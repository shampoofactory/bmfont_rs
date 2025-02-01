//! Binary packing/ unpacking implementations.
//!
//! Encoding types:
//! * `()`: untyped
//! * `C`: C type
//! * `V1`: version 1
//! * `V2`: version 2
//! * `V3`: version 3
//!
//! To maintain flexibility we do not implement additional `V*` traits where the underlying binary
//! encoding has not changed. We delegate selection logic to the client.
//! For example: `Kerning` has a `V1` encoding which is unchanged in `V2` and `V3`, we therefore
//! define `V1` only; the client is aware of this and will select `V1` in all cases.

use std::convert::TryFrom;

use crate::charset::Charset;
use crate::font::*;
use crate::parse::ParseError;

use super::bits::BitField;
use super::pack::{self, Pack, PackDyn, PackDynLen, PackLen, Unpack, UnpackDyn};

pub const SMOOTH: u32 = 7;
pub const UNICODE: u32 = 6;
pub const ITALIC: u32 = 5;
pub const BOLD: u32 = 4;
pub const FIXED_HEIGHT: u32 = 3;

pub const PACKED: u32 = 0;

macro_rules! pack_len {
    ($($u:ty),*) => {
        {
            0
            $(
                + std::mem::size_of::<$u>()
            )*
        }
    };
}

macro_rules! pack {
    ($dst:expr, $($u:expr),*) => {{
        #[allow(unused_assignments)]
        {
            use std::mem::size_of_val;

            let dst: &mut Vec<u8> = $dst;
            let mut len = 0;
            $(
                len += size_of_val($u);
            )*
            let mut off = dst.len();
            dst.resize(off + len, 0);
            {
                let dst = dst.as_mut_slice();
                $(
                    let bytes = $u.to_le_bytes();
                    let end = off + size_of_val($u);
                    (&mut dst[off..end]).copy_from_slice(bytes.as_ref());
                    off = end;
                )*
            }
        }
    }};
}

macro_rules! unpack {
    ($src:expr, $($u:ty),*) => {{
    #[allow(unused_assignments)]
    {
        use std::mem::size_of;

        let src: &[u8] = $src;
        let mut len = 0;
        $(
            len += size_of::<$u>();
        )*
        if src.len() < len {
            None
        } else {
            let mut off = 0;
            Some((
                    $({
                        let mut bytes = [0u8; size_of::<$u>()];
                        let end = off + size_of::<$u>();
                        bytes.as_mut().copy_from_slice(&src[off..end]);
                        off = end;
                        <$u>::from_le_bytes(bytes)
                    },)*
                ))
            }
        }
    }};
}

#[derive(Clone, Copy, Debug)]
pub struct V1;

#[derive(Clone, Copy, Debug)]
pub struct V2;

#[derive(Clone, Copy, Debug)]
pub struct V3;

#[derive(Clone, Copy, Debug)]
pub struct C;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Magic(pub u32);

impl Magic {
    pub const TORCH: u32 = 0x0046_4D42;

    pub fn new(version: u8) -> Self {
        Self((version as u32) << 24 | Self::TORCH)
    }

    pub fn version(self) -> crate::Result<u8> {
        if self.0 & 0x00FF_FFFF != Self::TORCH {
            Err(crate::Error::InvalidBinary { magic_bytes: self.0 })
        } else {
            Ok((self.0 >> 24) as u8)
        }
    }
}

impl PackLen for Magic {
    const PACK_LEN: usize = pack_len!(u32);
}

impl Pack for Magic {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()> {
        pack!(dst, &self.0);
        Ok(())
    }
}

impl Unpack for Magic {
    fn unpack(src: &[u8]) -> crate::Result<Self> {
        if let Some((magic,)) = unpack!(src, u32) {
            Ok(Self(magic))
        } else {
            pack::underflow()
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Block {
    pub id: u8,
    pub len: u32,
}

impl Block {
    pub const fn new(id: u8, len: u32) -> Self {
        Self { id, len }
    }
}

impl PackLen for Block {
    const PACK_LEN: usize = pack_len!(u8, u32);
}

impl Pack for Block {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()> {
        pack!(dst, &self.id, &self.len);
        Ok(())
    }
}

impl Unpack for Block {
    fn unpack(src: &[u8]) -> crate::Result<Self> {
        if let Some((id, len)) = unpack!(src, u8, u32) {
            Ok(Self { id, len })
        } else {
            pack::underflow()
        }
    }
}

impl PackDynLen<V2> for Info {
    const PACK_DYN_MIN: usize = pack_len!(i16, u8, u8, u16, u8, u8, u8, u8, u8, u8, u8, u8);

    #[inline(always)]
    fn dyn_len(&self) -> usize {
        <Info as PackDynLen<V2>>::PACK_DYN_MIN + <String as PackDynLen<C>>::dyn_len(&self.face)
    }
}

impl PackDyn<V2> for Info {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        let charset = if self.unicode {
            if self.charset == Charset::Null {
                0
            } else {
                return Err(crate::Error::InvalidBinaryEncoding {
                    unicode: self.unicode,
                    charset: self.charset.clone(),
                });
            }
        } else {
            match self.charset {
                Charset::Null => 0,
                Charset::Tagged(u) => u,
                Charset::Undefined(_) => 0,
            }
        };
        let mut bits = BitField(0);
        bits.set(SMOOTH, self.smooth);
        bits.set(UNICODE, self.unicode);
        bits.set(ITALIC, self.italic);
        bits.set(BOLD, self.bold);
        pack!(
            dst,
            &self.size,
            &bits.0,
            &charset,
            &self.stretch_h,
            &self.aa,
            &self.padding.up,
            &self.padding.right,
            &self.padding.down,
            &self.padding.left,
            &self.spacing.horizontal,
            &self.spacing.vertical,
            &self.outline
        );
        let face = c_string(self.face.as_bytes())?;
        dst.extend_from_slice(face);
        dst.push(0);
        Ok(dst.len() - mark)
    }
}

impl UnpackDyn<V2> for Info {
    fn unpack_dyn(src: &[u8]) -> crate::Result<(Self, usize)> {
        if let Some((
            size,
            bits,
            charset,
            stretch_h,
            aa,
            padding_up,
            padding_right,
            padding_down,
            padding_left,
            spacing_horiz,
            spacing_vert,
            outline,
        )) = unpack!(src, i16, u8, u8, u16, u8, u8, u8, u8, u8, u8, u8, u8)
        {
            let dyn_min = <Self as PackDynLen<V2>>::PACK_DYN_MIN;
            let src = &src[dyn_min..];
            let (face, face_len) = <String as UnpackDyn<C>>::unpack_dyn(src)?;
            let padding = Padding::new(padding_up, padding_right, padding_down, padding_left);
            let spacing = Spacing::new(spacing_horiz, spacing_vert);
            let bits = BitField(bits);
            let smooth = bits.get(SMOOTH);
            let unicode = bits.get(UNICODE);
            let italic = bits.get(ITALIC);
            let bold = bits.get(BOLD);
            let _fixed_height = bits.get(FIXED_HEIGHT);
            let charset =
                if unicode && charset == 0 { Charset::Null } else { Charset::Tagged(charset) };
            Ok((
                Self {
                    face,
                    size,
                    bold,
                    italic,
                    charset,
                    unicode,
                    stretch_h,
                    smooth,
                    aa,
                    padding,
                    spacing,
                    outline,
                },
                dyn_min + face_len,
            ))
        } else {
            pack::underflow()
        }
    }
}

impl PackLen<V3> for Common {
    const PACK_LEN: usize = pack_len!(u16, u16, u16, u16, u16, u8, u8, u8, u8, u8);
}

impl Pack<V3> for Common {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()> {
        let mut bits = BitField(0);
        bits.set(PACKED, self.packed);
        pack!(
            dst,
            &self.line_height,
            &self.base,
            &self.scale_w,
            &self.scale_h,
            &self.pages,
            &bits.0,
            &(self.alpha_chnl as u8),
            &(self.red_chnl as u8),
            &(self.green_chnl as u8),
            &(self.blue_chnl as u8)
        );
        Ok(())
    }
}

impl Unpack<V3> for Common {
    fn unpack(src: &[u8]) -> crate::Result<Self> {
        if let Some((
            line_height,
            base,
            scale_w,
            scale_h,
            pages,
            bits,
            alpha_chnl,
            red_chnl,
            green_chnl,
            blue_chnl,
        )) = unpack!(src, u16, u16, u16, u16, u16, u8, u8, u8, u8, u8)
        {
            let bits = BitField(bits);
            let packed = bits.get(PACKED);
            Ok(Self {
                line_height,
                base,
                scale_w,
                scale_h,
                pages,
                packed,
                alpha_chnl: parse_u8(alpha_chnl)?,
                red_chnl: parse_u8(red_chnl)?,
                green_chnl: parse_u8(green_chnl)?,
                blue_chnl: parse_u8(blue_chnl)?,
            })
        } else {
            pack::underflow()
        }
    }
}

impl PackLen<V1> for Char {
    const PACK_LEN: usize = pack_len!(u32, u16, u16, u16, u16, i16, i16, i16, u8, u8);
}

impl Pack<V1> for Char {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()> {
        pack!(
            dst,
            &self.id,
            &self.x,
            &self.y,
            &self.width,
            &self.height,
            &self.xoffset,
            &self.yoffset,
            &self.xadvance,
            &self.page,
            &u8::from(self.chnl)
        );
        Ok(())
    }
}

impl Unpack<V1> for Char {
    fn unpack(src: &[u8]) -> crate::Result<Self> {
        if let Some((id, x, y, width, height, xoffset, yoffset, xadvance, page, chnl)) =
            unpack!(src, u32, u16, u16, u16, u16, i16, i16, i16, u8, u8)
        {
            Ok(Self {
                id,
                x,
                y,
                width,
                height,
                xoffset,
                yoffset,
                xadvance,
                page,
                chnl: parse_u8(chnl)?,
            })
        } else {
            pack::underflow()
        }
    }
}

impl PackLen<V1> for Kerning {
    const PACK_LEN: usize = pack_len!(u32, u32, i16);
}

impl Pack<V1> for Kerning {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()> {
        pack!(dst, &self.first, &self.second, &self.amount);
        Ok(())
    }
}

impl Unpack<V1> for Kerning {
    fn unpack(src: &[u8]) -> crate::Result<Self> {
        if let Some((first, second, amount)) = unpack!(src, u32, u32, i16) {
            Ok(Self { first, second, amount })
        } else {
            pack::underflow()
        }
    }
}

impl PackDynLen<C> for &str {
    const PACK_DYN_MIN: usize = 1;

    #[inline(always)]
    fn dyn_len(&self) -> usize {
        self.len() + 1
    }
}

impl PackDyn<C> for &str {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let bytes = c_string(self.as_bytes())?;
        dst.extend_from_slice(bytes);
        dst.push(0);
        Ok(bytes.len() + 1)
    }
}

impl PackDynLen<C> for String {
    const PACK_DYN_MIN: usize = 1;

    #[inline(always)]
    fn dyn_len(&self) -> usize {
        self.len() + 1
    }
}

impl UnpackDyn<C> for String {
    fn unpack_dyn(src: &[u8]) -> crate::Result<(Self, usize)> {
        let mut i = 0;
        while i < src.len() {
            if src[i] == 0 {
                let string = utf8_string((&src[..i]).into())?;
                return Ok((string, i + 1));
            }
            i += 1;
        }
        Err(crate::Error::Parse {
            line: None,
            entity: "CString".to_owned(),
            err: "missing NUL".to_owned(),
        })
    }
}

fn c_string(bytes: &[u8]) -> crate::Result<&[u8]> {
    if bytes.contains(&0) {
        Err(crate::Error::Parse {
            line: None,
            entity: "CString".to_owned(),
            err: "contains NUL".to_owned(),
        })
    } else {
        Ok(bytes)
    }
}

fn utf8_string(vec: Vec<u8>) -> crate::Result<String> {
    match String::from_utf8(vec) {
        Ok(u) => Ok(u),
        Err(e) => {
            Err(crate::Error::Parse { line: None, entity: "String".to_owned(), err: e.to_string() })
        }
    }
}

fn parse_u8<T: TryFrom<u8, Error = ParseError>>(u: u8) -> crate::Result<T> {
    T::try_from(u).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "String".to_owned(),
        err: e.to_string(),
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! test_pack {
        ($name:ident, $obj:ty, $t:ty, $src:expr, $dst:expr) => {
            mod $name {
                use super::*;

                #[test]
                fn pack_dyn() -> crate::Result<()> {
                    let mut dst = Vec::default();
                    Pack::<$t>::pack($src, &mut dst)?;
                    assert_eq!(dst, $dst);
                    Ok(())
                }

                #[test]
                fn unpack_dyn() -> crate::Result<()> {
                    let dst: &[u8] = $dst;
                    let obj = <$obj as Unpack<$t>>::unpack(dst)?;
                    assert_eq!(&obj, $src);
                    Ok(())
                }
            }
        };
    }

    test_pack!(magic, Magic, (), &Magic::new(0x03), &[0x42, 0x4D, 0x46, 0x03]);
    test_pack!(block, Block, (), &Block::new(0x01, 0x02030405), &[0x01, 0x05, 0x04, 0x03, 0x02]);
    test_pack!(
        common_v3,
        Common,
        V3,
        &Common::new(
            16,
            32,
            64,
            128,
            4,
            true,
            Packing::Outline,
            Packing::GlyphOutline,
            Packing::Zero,
            Packing::One
        ),
        &[
            0x10, 0x00, // lineHeight
            0x20, 0x00, // base,
            0x40, 0x00, // scaleW
            0x80, 0x00, // scaleH
            0x04, 0x00, // pages
            0x01, // bitField
            0x01, // alphaChnl
            0x02, // redChnl
            0x03, // blueChnl
            0x04, // greenChnl
        ]
    );
    test_pack!(
        char_v1,
        Char,
        V1,
        &Char::new(1, 4, 8, 16, 32, 64, 128, 256, 0, Chnl::ALL),
        &[
            0x01, 0x00, 0x00, 0x00, // id
            0x04, 0x00, // x
            0x08, 0x00, // y
            0x10, 0x00, // width
            0x20, 0x00, // height
            0x40, 0x00, // xoffset
            0x80, 0x00, // yoffset
            0x00, 0x01, // xadvance
            0x00, // xadvance
            0x0F, // chnl
        ]
    );
    test_pack!(
        kerning_v1,
        Kerning,
        V1,
        &Kerning::new(1, 2, -1),
        &[
            0x01, 0x00, 0x00, 0x00, // first
            0x02, 0x00, 0x00, 0x00, // second
            0xFF, 0xFF // amount
        ]
    );

    macro_rules! test_pack_dyn {
        ($name:ident, $obj:ty, $t:ty, $src:expr, $dst:expr) => {
            mod $name {
                use super::*;

                #[test]
                fn pack_dyn() -> crate::Result<()> {
                    let mut dst = Vec::default();
                    let pack_len = PackDyn::<$t>::pack_dyn($src, &mut dst)?;
                    assert_eq!(dst.len(), pack_len);
                    assert_eq!(dst, $dst);
                    Ok(())
                }

                #[test]
                fn unpack_dyn() -> crate::Result<()> {
                    let dst: &[u8] = $dst;
                    let (obj, obj_len) = <$obj as UnpackDyn<$t>>::unpack_dyn(dst)?;
                    assert_eq!(obj_len, dst.len());
                    assert_eq!(&obj, $src);
                    Ok(())
                }
            }
        };
    }

    test_pack_dyn!(
        info_v2,
        Info,
        V2,
        &Info::new(
            "Arial".to_owned(),
            32,
            true,
            true,
            Charset::Null,
            true,
            100,
            true,
            4,
            Padding::new(1, 2, 3, 4),
            Spacing::new(5, 6),
            7
        ),
        &[
            0x20, 0x00, // fontSize
            0xF0, // bitField
            0x00, // charSet
            0x64, 0x00, // stretchH
            0x04, // aa
            0x01, // paddingUp
            0x02, // paddingRight
            0x03, // paddingDown
            0x04, // paddingLeft
            0x05, // spacingHoriz
            0x06, // spacingHoriz
            0x07, // outline
            0x41, 0x72, 0x69, 0x61, 0x6C, 0x00, // fontName
        ]
    );

    test_pack_dyn!(string_c, String, C, &"test", &[0x74, 0x65, 0x73, 0x74, 0x00]);
    test_pack_dyn!(string_c_null, String, C, &"", &[0]);
}
