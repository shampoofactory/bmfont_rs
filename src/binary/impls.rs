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
//!
//! Note. These methods have limited validation checks, they are primarily concerned with the
//! packing and unpacking of data structures, not with the validity of their contents.
use std::convert::TryFrom;

use crate::binary::constants::{CHARS, COMMON, INFO, KERNING_PAIRS, PAGES};
use crate::builder::FontProto;
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
    // $src: &mut [u8]
    ($src:expr, $($u:ty),*) => {{
    #[allow(unused_assignments)]
    {
        use std::mem::size_of;

        let mut len = 0;
        $(
            len += size_of::<$u>();
        )*
        if $src.len() < len {
            pack::underflow()
        } else {
            Ok((
                    $({
                        let mut bytes = [0u8; size_of::<$u>()];
                        let (obj, overflow) = $src.split_at(size_of::<$u>());
                        bytes.as_mut().copy_from_slice(obj);
                        *$src = overflow;
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
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        pack!(dst, &self.0);
        Ok(Self::PACK_LEN)
    }
}

impl Unpack for Magic {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self> {
        unpack!(src, u32).map(|(magic,)| Self(magic))
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
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        pack!(dst, &self.id, &self.len);
        Ok(Self::PACK_LEN)
    }
}

impl Unpack for Block {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self> {
        unpack!(src, u8, u32).map(|(id, len)| Self { id, len })
    }
}

impl PackDynLen<V3> for Font {
    const PACK_DYN_MIN: usize =
        Magic::PACK_LEN + Block::PACK_LEN * 4 + Info::PACK_DYN_MIN + Common::PACK_LEN;

    fn dyn_len(&self) -> usize {
        Magic::PACK_LEN
            + <Common as PackLen<V3>>::PACK_LEN
            + Block::PACK_LEN * 4
            + PackDynLen::<V2>::dyn_len(&self.info)
            + PackDynLen::<C>::dyn_len(&self.pages)
            + PackDynLen::<V1>::dyn_len(&self.chars)
            + (if !self.kernings.is_empty() {
                Block::PACK_LEN + PackDynLen::<V1>::dyn_len(&self.kernings)
            } else {
                0
            })
    }
}

impl PackDyn<V3> for Font {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        // Magic V3
        Magic::new(3).pack(dst)?;
        // Info V2
        Block::new(INFO, PackDynLen::<V2>::dyn_len(&self.info) as u32).pack(dst)?;
        PackDyn::<V2>::pack_dyn(&self.info, dst)?;
        // Common V3
        Block::new(COMMON, <Common as PackLen<V3>>::PACK_LEN as u32).pack(dst)?;
        Pack::<V3>::pack(&self.common, dst)?;
        // Pages C
        Block::new(PAGES, PackDynLen::<C>::dyn_len(&self.pages) as u32).pack(dst)?;
        PackDyn::<C>::pack_dyn(&self.pages, dst)?;
        // Chars V1
        Block::new(CHARS, PackDynLen::<V1>::dyn_len(&self.chars) as u32).pack(dst)?;
        PackDyn::<V1>::pack_dyn(&self.chars, dst)?;
        // Kernings V1 optional
        if !self.kernings.is_empty() {
            Block::new(KERNING_PAIRS, PackDynLen::<V1>::dyn_len(&self.kernings) as u32)
                .pack(dst)?;
            PackDyn::<V1>::pack_dyn(&self.kernings, dst)?;
        }
        Ok(dst.len() - mark)
    }
}

impl UnpackDyn<V3> for Font {
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        let version = Magic::unpack_next(src)?.version()?;
        if version != 3 {
            return Err(crate::Error::UnsupportedBinaryVersion { version });
        }
        let mut proto = FontProto::default();
        while !src.is_empty() {
            let Block { id, len } = Block::unpack_next(src)?;
            if len as usize > src.len() {
                return pack::underflow();
            }
            let (mut block, overflow) = src.split_at(len as usize);
            *src = overflow;
            match id {
                INFO => {
                    proto.set_info(None, <_ as UnpackDyn<V2>>::unpack_dyn(&mut block)?)?;
                }
                COMMON => {
                    proto.set_common(None, <_ as Unpack<V3>>::unpack(&mut block)?)?;
                }
                PAGES => {
                    proto.set_pages(None, <_ as UnpackDyn<C>>::unpack_dyn(&mut block)?)?;
                }
                CHARS => {
                    proto.set_chars(None, <_ as UnpackDyn<V1>>::unpack_dyn(&mut block)?)?;
                }
                KERNING_PAIRS => {
                    proto.set_kernings(None, <_ as UnpackDyn<V1>>::unpack_dyn(&mut block)?)?;
                }
                id => return Err(crate::Error::InvalidBinaryBlock { id }),
            }
        }
        proto.build_unchecked()
    }
}

impl PackDynLen<V2> for Info {
    const PACK_DYN_MIN: usize = pack_len!(i16, u8, u8, u16, u8, u8, u8, u8, u8, u8, u8, u8);

    #[inline(always)]
    fn dyn_len(&self) -> usize {
        <Info as PackDynLen<V2>>::PACK_DYN_MIN + PackDynLen::<C>::dyn_len(&self.face)
    }
}

impl PackDyn<V2> for Info {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        let charset = match self.charset {
            Charset::Null | Charset::Undefined(_) => 0,
            Charset::Tagged(u) => u,
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
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        match unpack!(src, i16, u8, u8, u16, u8, u8, u8, u8, u8, u8, u8, u8) {
            Ok((
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
            )) => {
                let face = UnpackDyn::<C>::unpack_dyn(src)?;
                let padding = Padding::new(padding_up, padding_right, padding_down, padding_left);
                let spacing = Spacing::new(spacing_horiz, spacing_vert);
                let bits = BitField(bits);
                let smooth = bits.get(SMOOTH);
                let unicode = bits.get(UNICODE);
                let italic = bits.get(ITALIC);
                let bold = bits.get(BOLD);
                let _fixed_height = bits.get(FIXED_HEIGHT);
                let charset = match charset {
                    0 if unicode => Charset::Null,
                    u => Charset::Tagged(u),
                };
                Ok(Self {
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
                })
            }
            Err(err) => Err(err),
        }
    }
}

impl PackLen<V3> for Common {
    const PACK_LEN: usize = pack_len!(u16, u16, u16, u16, u16, u8, u8, u8, u8, u8);
}

impl Pack<V3> for Common {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
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
        Ok(Self::PACK_LEN)
    }
}

impl Unpack<V3> for Common {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self> {
        match unpack!(src, u16, u16, u16, u16, u16, u8, u8, u8, u8, u8) {
            Ok((
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
            )) => {
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
            }
            Err(err) => Err(err),
        }
    }
}

impl PackDynLen<C> for Vec<String> {
    const PACK_DYN_MIN: usize = 0;

    fn dyn_len(&self) -> usize {
        self.iter().map(PackDynLen::<C>::dyn_len).sum()
    }
}

impl PackDyn<C> for Vec<String> {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        for s in self.iter() {
            PackDyn::<C>::pack_dyn(&s.as_str(), dst)?;
        }
        Ok(dst.len() - mark)
    }
}

impl UnpackDyn<C> for Vec<String> {
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        let mut dst = Vec::default();
        <String as UnpackDyn<C>>::unpack_dyn_all(src, |file| {
            dst.push(file);
            Ok(())
        })?;
        Ok(dst)
    }
}

impl PackDynLen<V1> for Vec<Char> {
    const PACK_DYN_MIN: usize = 0;

    fn dyn_len(&self) -> usize {
        <Char as PackLen<V1>>::PACK_LEN * self.len()
    }
}

impl PackDyn<V1> for Vec<Char> {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        for char in self.iter() {
            Pack::<V1>::pack(char, dst)?;
        }
        Ok(dst.len() - mark)
    }
}

impl UnpackDyn<V1> for Vec<Char> {
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        let mut dst = Vec::default();
        <Char as Unpack<V1>>::unpack_all(src, |file| {
            dst.push(file);
            Ok(())
        })?;
        Ok(dst)
    }
}

impl PackDynLen<V1> for Vec<Kerning> {
    const PACK_DYN_MIN: usize = 0;

    fn dyn_len(&self) -> usize {
        <Kerning as PackLen<V1>>::PACK_LEN * self.len()
    }
}

impl PackDyn<V1> for Vec<Kerning> {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        let mark = dst.len();
        for kerning in self.iter() {
            Pack::<V1>::pack(kerning, dst)?;
        }
        Ok(dst.len() - mark)
    }
}

impl UnpackDyn<V1> for Vec<Kerning> {
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        let mut dst = Vec::default();
        <Kerning as Unpack<V1>>::unpack_all(src, |file| {
            dst.push(file);
            Ok(())
        })?;
        Ok(dst)
    }
}

impl PackLen<V1> for Char {
    const PACK_LEN: usize = pack_len!(u32, u16, u16, u16, u16, i16, i16, i16, u8, u8);
}

impl Pack<V1> for Char {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
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
        Ok(Self::PACK_LEN)
    }
}

impl Unpack<V1> for Char {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self> {
        match unpack!(src, u32, u16, u16, u16, u16, i16, i16, i16, u8, u8) {
            Ok((id, x, y, width, height, xoffset, yoffset, xadvance, page, chnl)) => Ok(Self {
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
            }),
            Err(err) => Err(err),
        }
    }
}

impl PackLen<V1> for Kerning {
    const PACK_LEN: usize = pack_len!(u32, u32, i16);
}

impl Pack<V1> for Kerning {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize> {
        pack!(dst, &self.first, &self.second, &self.amount);
        Ok(Self::PACK_LEN)
    }
}

impl Unpack<V1> for Kerning {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self> {
        match unpack!(src, u32, u32, i16) {
            Ok((first, second, amount)) => Ok(Self { first, second, amount }),
            Err(err) => Err(err),
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
        let mark = dst.len();
        let bytes = c_string(self.as_bytes())?;
        dst.extend_from_slice(bytes);
        dst.push(0);
        Ok(dst.len() - mark)
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
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self> {
        let mut i = 0;
        while i < src.len() {
            if src[i] == 0 {
                let (obj, overflow) = src.split_at(i);
                let string = utf8_string((obj).into())?;
                *src = &overflow[1..];
                return Ok(string);
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
                fn pack() -> crate::Result<()> {
                    let mut dst = Vec::default();
                    let pack_len = Pack::<$t>::pack($src, &mut dst)?;
                    assert_eq!($dst.len(), pack_len);
                    assert_eq!(pack_len, <$obj as PackLen<$t>>::PACK_LEN);
                    assert_eq!(dst, $dst);
                    Ok(())
                }

                #[test]
                fn unpack() -> crate::Result<()> {
                    let dst = &mut $dst.as_slice();
                    let obj = <$obj as Unpack<$t>>::unpack(dst)?;
                    assert_eq!(&obj, $src);
                    assert!(dst.is_empty());
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
                    assert_eq!(PackDynLen::<$t>::dyn_len($src), pack_len);
                    assert_eq!(dst, $dst);
                    Ok(())
                }

                #[test]
                fn unpack_dyn() -> crate::Result<()> {
                    let dst = &mut $dst.as_slice();
                    let obj = <$obj as UnpackDyn<$t>>::unpack_dyn(dst)?;
                    assert!(dst.is_empty());
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
    test_pack_dyn!(
        vec_string_c,
        Vec<String>,
        C,
        &vec!["test".to_owned()],
        &[0x74, 0x65, 0x73, 0x74, 0x00]
    );
    test_pack_dyn!(
        vec_string_c_3,
        Vec<String>,
        C,
        &vec!["abc".to_owned(), "de".to_owned(), "f".to_owned()],
        &[0x61, 0x62, 0x63, 0x00, 0x64, 0x65, 0x00, 0x66, 0x00]
    );
    test_pack_dyn!(string_c, String, C, &"test", &[0x74, 0x65, 0x73, 0x74, 0x00]);
    test_pack_dyn!(string_c_null, String, C, &"", &[0]);
}
