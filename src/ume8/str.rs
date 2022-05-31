use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use crate::ume8::string::String as Ume8String;

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
        unsafe { &*(inner as *const [u8] as *const Str) }
    }

    #[allow(unused_unsafe)]
    pub(crate) unsafe fn from_inner_mut(inner: &mut [u8]) -> &mut Self {
        unsafe { &mut *(inner as *mut [u8] as *mut Str) }
    }
}

impl Str {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
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
