use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use crate::ume8::decode::{DecodeUnchecked, ToCharUnchecked};
use crate::ume8::string::String as Ume8String;
use crate::ume8::util::is_singleton;

#[repr(transparent)]
#[derive(PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Str {
    bytes: [u8],
}

impl Str {
    pub fn new<S: AsRef<Self> + ?Sized>(s: &S) -> &Self {
        s.as_ref()
    }

    #[allow(unused_unsafe)]
    pub(crate) unsafe fn from_inner(inner: &[u8]) -> &Self {
        unsafe { std::mem::transmute(inner) }
    }

    #[allow(unused_unsafe)]
    pub(crate) unsafe fn from_inner_mut(inner: &mut [u8]) -> &mut Self {
        unsafe { std::mem::transmute(inner) }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    pub fn chars(&self) -> ToCharUnchecked<DecodeUnchecked<std::vec::IntoIter<u8>>> {
        unsafe {
            ToCharUnchecked::new(
                DecodeUnchecked::new(
                    self.bytes.to_owned().into_iter()
                )
            )
        }
    }

    // TODO
    // pub fn lines(&self) -> Iterator {
    //
    // }

    pub fn contains(&self, other: &Self) -> bool {
        if other.bytes.len() == 0 {
            return true;
        }

        self.bytes.windows(other.bytes.len())
            .any(|chunk| &other.bytes == chunk)
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.bytes.starts_with(&other.bytes)
    }

    pub fn ends_with(&self, other: &Self) -> bool {
        self.bytes.ends_with(&other.bytes)
    }

    // TODO
    // pub fn split(&self) -> Iterator {
    //
    // }

    // TODO
    // pub fn trim(&self) -> Self {
    //
    // }

    // TODO
    // pub fn trim_start(&self) -> Self {
    //
    // }

    // TODO
    // pub fn trim_left(&self) -> Self {
    //
    // }

    // TODO
    // pub fn parse(&self) -> T {
    //
    // }

    pub fn is_ascii(&self) -> bool {
        self.bytes.iter()
            .all(|byte| is_singleton(byte))
    }
}

impl Default for &Str {
    fn default() -> Self {
        unsafe { Str::from_inner(&[]) }
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: Ume8String = self.into();
        Display::fmt(&string, f)
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: Ume8String = self.into();
        Debug::fmt(&string, f)
    }
}

impl Borrow<Str> for Ume8String {
    fn borrow(&self) -> &Str {
        &self[..]
    }
}

impl ToOwned for Str {
    type Owned = Ume8String;

    fn to_owned(&self) -> Self::Owned {
        unsafe { Ume8String::from_bytes_unchecked(self.bytes.to_owned()) }
    }
}

impl AsRef<Str> for Str {
    fn as_ref(&self) -> &Str {
        self
    }
}

impl AsRef<Str> for Ume8String {
    fn as_ref(&self) -> &Str {
        self
    }
}
