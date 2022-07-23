use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut, RangeFull};
use std::str::FromStr;
use std::string::String;
use crate::ume8::encode::{EncodeUnchecked, EncodeSequenceUnchecked};
use crate::ume8::str::Ume8Str;

#[repr(transparent)]
#[derive(PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Ume8String {
    bytes: Vec<u8>,
}

impl Ume8String {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Self {
        Self {
            bytes
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.bytes.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.bytes.shrink_to(min_capacity);
    }

    pub fn push(&mut self, ch: char) {
        self.bytes.extend(
            EncodeSequenceUnchecked::new(ch as u32)
        );
    }

    // TODO
    // pub fn truncate(&mut self, length: usize) {
    //
    // }

    // TODO
    // pub fn pop(&mut self) -> Option<char> {
    //     let ch = self.chars().rev().next()?;
    //     let newlen = self.len() - ch.len_utf8();
    //     unsafe {
    //         self.vec.set_len(newlen);
    //     }
    //     Some(ch)
    // }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &Ume8Str {
        self
    }

    #[inline]
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut Ume8Str {
        self
    }
}

impl Ume8String {
    pub fn push_string(&mut self, string: Self) {
        self.bytes.extend(string.bytes);
    }
}

impl Deref for Ume8String {
    type Target = Ume8Str;

    fn deref(&self) -> &Self::Target {
        &self[..]
    }
}

impl DerefMut for Ume8String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self[..]
    }
}

impl AsRef<Ume8Str> for Ume8String {
    fn as_ref(&self) -> &Ume8Str {
        self
    }
}

impl AsMut<Ume8Str> for Ume8String {
    fn as_mut(&mut self) -> &mut Ume8Str {
        self
    }
}

impl Borrow<Ume8Str> for Ume8String {
    fn borrow(&self) -> &Ume8Str {
        self.deref()
    }
}

impl BorrowMut<Ume8Str> for Ume8String {
    fn borrow_mut(&mut self) -> &mut Ume8Str {
        self.deref_mut()
    }
}

impl Index<RangeFull> for Ume8String {
    type Output = Ume8Str;

    fn index(&self, _index: RangeFull) -> &Self::Output {
        unsafe { Ume8Str::from_inner(&self.bytes) }
    }
}

impl IndexMut<RangeFull> for Ume8String {
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        unsafe { Ume8Str::from_inner_mut(&mut self.bytes) }
    }
}

impl Clone for Ume8String {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.bytes.clone_from(&source.bytes);
    }
}

impl Default for Ume8String {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Ume8String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

impl Debug for Ume8String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_ref(), f)
    }
}

impl Extend<Ume8String> for Ume8String {
    fn extend<T: IntoIterator<Item=Ume8String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend(string.bytes));
    }
}

impl <'a> Extend<&'a Ume8String> for Ume8String {
    fn extend<T: IntoIterator<Item=&'a Ume8String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend_from_slice(&string.bytes));
    }
}

impl Extend<char> for Ume8String {
    fn extend<T: IntoIterator<Item=char>>(&mut self, iter: T) {
        self.bytes.extend(
            EncodeUnchecked::new(
                iter.into_iter()
                    .map(|data| data as u32)
            )
        );
    }
}

impl <'a> Extend<&'a char> for Ume8String {
    fn extend<T: IntoIterator<Item=&'a char>>(&mut self, iter: T) {
        self.bytes.extend(
            EncodeUnchecked::new(
                iter.into_iter()
                    .map(|data| *data as u32)
            )
        );
    }
}

impl <I> FromIterator<I> for Ume8String
    where Ume8String: Extend<I>
{
    fn from_iter<T: IntoIterator<Item=I>>(iter: T) -> Self {
        let mut string = Ume8String::new();
        string.extend(iter);
        string
    }
}

impl From<&Ume8Str> for Ume8String {
    fn from(s: &Ume8Str) -> Ume8String {
        s.to_owned()
    }
}

impl FromStr for Ume8String {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Ume8String, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for Ume8String {
    fn from(s: &str) -> Ume8String {
        let mut string = Ume8String::with_capacity(s.len());
        string.extend(s.chars());
        string
    }
}

impl From<String> for Ume8String {
    fn from(s: String) -> Ume8String {
        s.as_str().into()
    }
}

impl From<&String> for Ume8String {
    fn from(s: &String) -> Ume8String {
        s.as_str().into()
    }
}

impl From<char> for Ume8String {
    fn from(char: char) -> Ume8String {
        let mut string = Ume8String::new();
        string.push(char);
        string
    }
}

impl From<&char> for Ume8String {
    fn from(char: &char) -> Ume8String {
        (*char).into()
    }
}

impl From<Ume8String> for String {
    fn from(s: Ume8String) -> String {
        s.as_str().into()
    }
}

impl From<&Ume8String> for String {
    fn from(s: &Ume8String) -> String {
        s.as_str().into()
    }
}
