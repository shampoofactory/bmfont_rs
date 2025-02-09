//! Binary packing/ unpacking traits.
//!
//! We define static (fixed length) and dynamic (variable length) packing and packing traits:
//! * Static: `PackLen`, `Pack` and `Unpack`
//! * Dynamic: `PackDynLen`, `PackDyn` and `UnpackDyn`
//!
//! These packing traits are generic over the encoding type.

pub trait PackLen<T = ()>: Sized {
    const PACK_LEN: usize;
}

pub trait Pack<T = ()>: PackLen<T> {
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<()>;
}

pub trait Unpack<T = ()>: PackLen<T> + Sized {
    fn unpack(src: &[u8]) -> crate::Result<Self>;

    #[inline(always)]
    fn unpack_take(src: &mut &[u8]) -> crate::Result<Self> {
        if src.len() < Self::PACK_LEN {
            return underflow();
        }
        let (obj, overflow) = src.split_at(Self::PACK_LEN);
        *src = overflow;
        Self::unpack(obj)
    }

    #[inline(always)]
    fn unpack_take_all<F>(mut src: &[u8], mut take: F) -> crate::Result<()>
    where
        F: FnMut(Self) -> crate::Result<()>,
    {
        while !src.is_empty() {
            take(Self::unpack_take(&mut src)?)?;
        }
        Ok(())
    }
}

pub trait PackDynLen<T = ()>: Sized {
    const PACK_DYN_MIN: usize;

    fn dyn_len(&self) -> usize;
}

pub trait PackDyn<T = ()>: PackDynLen<T> {
    fn pack_dyn(&self, dst: &mut Vec<u8>) -> crate::Result<usize>;
}

pub trait UnpackDyn<T = ()>: PackDynLen<T> + Sized {
    fn unpack_dyn(src: &[u8]) -> crate::Result<(Self, usize)>;

    #[inline(always)]
    fn unpack_dyn_take(src: &mut &[u8]) -> crate::Result<Self> {
        let (obj, obj_len) = Self::unpack_dyn(src)?;
        if obj_len <= src.len() {
            *src = &src[obj_len..];
            Ok(obj)
        } else {
            overflow()
        }
    }

    #[inline(always)]
    fn unpack_dyn_take_all<F>(mut src: &[u8], mut take: F) -> crate::Result<()>
    where
        F: FnMut(Self) -> crate::Result<()>,
    {
        while !src.is_empty() {
            take(Self::unpack_dyn_take(&mut src)?)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn unpack_dyn_tight(src: &[u8]) -> crate::Result<Self> {
        let (obj, obj_len) = Self::unpack_dyn(src)?;
        if obj_len == src.len() {
            Ok(obj)
        } else {
            overflow()
        }
    }
}

pub fn overflow<T>() -> crate::Result<T> {
    Err(crate::Error::Parse { line: None, entity: "buffer".to_owned(), err: "overflow".to_owned() })
}

pub fn underflow<T>() -> crate::Result<T> {
    Err(crate::Error::Parse {
        line: None,
        entity: "buffer".to_owned(),
        err: "underflow".to_owned(),
    })
}
