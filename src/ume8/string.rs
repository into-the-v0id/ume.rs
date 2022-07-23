use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut, RangeFull};
use std::str::FromStr;
use std::string::String as StdString;
use crate::ume8::encode::{EncodeUnchecked, EncodeSequenceUnchecked};
use crate::ume8::str::Str;

#[repr(transparent)]
#[derive(PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct String {
    bytes: Vec<u8>,
}

impl String {
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
    pub fn as_str(&self) -> &Str {
        self
    }

    #[inline]
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut Str {
        self
    }
}

impl String {
    pub fn push_string(&mut self, string: Self) {
        self.bytes.extend(string.bytes);
    }
}

impl Deref for String {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        &self[..]
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self[..]
    }
}

impl Index<RangeFull> for String {
    type Output = Str;

    fn index(&self, _index: RangeFull) -> &Self::Output {
        unsafe { Str::from_inner(&self.bytes) }
    }
}

impl IndexMut<RangeFull> for String {
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        unsafe { Str::from_inner_mut(&mut self.bytes) }
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.bytes.clone_from(&source.bytes);
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: StdString = self.into();
        Display::fmt(&string, f)
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: StdString = self.into();
        Debug::fmt(&string, f)
    }
}

impl Extend<String> for String {
    fn extend<T: IntoIterator<Item=String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend(string.bytes));
    }
}

impl <'a> Extend<&'a String> for String {
    fn extend<T: IntoIterator<Item=&'a String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend_from_slice(&string.bytes));
    }
}

impl Extend<char> for String {
    fn extend<T: IntoIterator<Item=char>>(&mut self, iter: T) {
        self.bytes.extend(
            EncodeUnchecked::new(
                iter.into_iter()
                    .map(|data| data as u32)
            )
        );
    }
}

impl <'a> Extend<&'a char> for String {
    fn extend<T: IntoIterator<Item=&'a char>>(&mut self, iter: T) {
        self.bytes.extend(
            EncodeUnchecked::new(
                iter.into_iter()
                    .map(|data| *data as u32)
            )
        );
    }
}

impl <I> FromIterator<I> for String
    where String: Extend<I>
{
    fn from_iter<T: IntoIterator<Item=I>>(iter: T) -> Self {
        let mut string = String::new();
        string.extend(iter);
        string
    }
}

impl From<&Str> for String {
    fn from(s: &Str) -> String {
        s.to_owned()
    }
}

impl FromStr for String {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<String, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for String {
    fn from(s: &str) -> String {
        let mut string = String::with_capacity(s.len());
        string.extend(s.chars());
        string
    }
}

impl From<StdString> for String {
    fn from(s: StdString) -> String {
        s.as_str().into()
    }
}

impl From<&StdString> for String {
    fn from(s: &StdString) -> String {
        s.as_str().into()
    }
}

impl From<char> for String {
    fn from(char: char) -> String {
        let mut string = String::new();
        string.push(char);
        string
    }
}

impl From<&char> for String {
    fn from(char: &char) -> String {
        (*char).into()
    }
}

impl From<&Str> for StdString {
    fn from(s: &Str) -> StdString {
        let mut string = StdString::with_capacity(s.len());
        string.extend(s.chars());
        string
    }
}

impl From<String> for StdString {
    fn from(s: String) -> StdString {
        s.as_str().into()
    }
}

impl From<&String> for StdString {
    fn from(s: &String) -> StdString {
        s.as_str().into()
    }
}
