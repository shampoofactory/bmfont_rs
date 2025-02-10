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
    fn pack(&self, dst: &mut Vec<u8>) -> crate::Result<usize>;
}

pub trait Unpack<T = ()>: PackLen<T> + Sized {
    fn unpack_next(src: &mut &[u8]) -> crate::Result<Self>;

    #[inline(always)]
    fn unpack(src: &mut &[u8]) -> crate::Result<Self> {
        let obj = Self::unpack_next(src)?;
        if src.is_empty() {
            Ok(obj)
        } else {
            overflow()
        }
    }

    #[inline(always)]
    fn unpack_all<F>(src: &mut &[u8], mut take: F) -> crate::Result<()>
    where
        F: FnMut(Self) -> crate::Result<()>,
    {
        while !src.is_empty() {
            take(Self::unpack_next(src)?)?;
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
    fn unpack_dyn_next(src: &mut &[u8]) -> crate::Result<Self>;

    #[inline(always)]
    fn unpack_dyn(src: &mut &[u8]) -> crate::Result<Self> {
        let obj = Self::unpack_dyn_next(src)?;
        if src.is_empty() {
            Ok(obj)
        } else {
            overflow()
        }
    }

    #[inline(always)]
    fn unpack_dyn_all<F>(src: &mut &[u8], mut take: F) -> crate::Result<()>
    where
        F: FnMut(Self) -> crate::Result<()>,
    {
        while !src.is_empty() {
            take(Self::unpack_dyn_next(src)?)?;
        }
        Ok(())
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
