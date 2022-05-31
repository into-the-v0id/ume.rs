use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::string::String as StdString;
use crate::EncodeUnchecked;
use crate::ume8::decode::{DecodeUnchecked, ToCharUnchecked};
use crate::ume8::encode::EncodeSequenceUnchecked;
use crate::ume8::util::is_singleton;

#[repr(transparent)]
#[derive(PartialOrd, PartialEq, Ord, Eq)]
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
}

impl String {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn chars(&self) -> ToCharUnchecked<DecodeUnchecked<std::vec::IntoIter<u8>>> {
        unsafe {
            ToCharUnchecked::new(
                DecodeUnchecked::new(
                    self.bytes.clone().into_iter()
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

impl String {
    pub fn push_string(&mut self, string: Self) {
        self.bytes.extend(string.bytes);
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

impl FromStr for String {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<String, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for String {
    fn from(s: &str) -> String {
        s.chars().collect::<String>()
    }
}

impl From<StdString> for String {
    fn from(s: StdString) -> String {
        s.chars().collect::<String>()
    }
}

impl From<&StdString> for String {
    fn from(s: &StdString) -> String {
        s.chars().collect::<String>()
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
        let mut string = String::new();
        string.push(*char);
        string
    }
}

impl From<String> for StdString {
    fn from(s: String) -> StdString {
        s.chars().collect::<StdString>()
    }
}

impl From<&String> for StdString {
    fn from(s: &String) -> StdString {
        s.chars().collect::<StdString>()
    }
}
