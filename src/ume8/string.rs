use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::EncodeUnchecked;
use crate::ume8::decode::{DecodeUnchecked, ToCharUnchecked};
use crate::ume8::encode::EncodeSequenceUnchecked;

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
        self.bytes.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.bytes.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.bytes.shrink_to(min_capacity)
    }

    pub fn push(&mut self, ch: char) {
        self.bytes.extend(
            EncodeSequenceUnchecked::new(ch as u32)
        );
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    // TODO
    // pub fn pop(&mut self) -> Option<char> {
    //     let ch = self.chars().rev().next()?;
    //     let newlen = self.len() - ch.len_utf8();
    //     unsafe {
    //         self.vec.set_len(newlen);
    //     }
    //     Some(ch)
    // }

    // pub fn len(&self) -> usize {
    //     self.bytes.len()
    // }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn clear(&mut self) {
        self.bytes.clear()
    }
}

impl String {
    pub fn chars(&self) -> ToCharUnchecked<DecodeUnchecked<std::vec::IntoIter<u8>>> {
        unsafe {
            ToCharUnchecked::new(
                DecodeUnchecked::new(
                    self.bytes.clone().into_iter()
                )
            )
        }
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
        let string: std::string::String = self.into();
        Display::fmt(&string, f)
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: std::string::String = self.into();
        Debug::fmt(&string, f)
    }
}

impl Extend<String> for String {
    fn extend<T: IntoIterator<Item=String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend(string.bytes))
    }
}

impl <'a> Extend<&'a String> for String {
    fn extend<T: IntoIterator<Item=&'a String>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|string| self.bytes.extend_from_slice(&string.bytes))
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

impl <'a, I: Into<&'a str>> From<I> for String {
    fn from(s: I) -> String {
        s.into().chars().collect::<String>()
    }
}

impl From<&String> for std::string::String {
    fn from(s: &String) -> std::string::String {
        s.chars().collect::<std::string::String>()
    }
}

impl From<String> for std::string::String {
    fn from(s: String) -> std::string::String {
        s.chars().collect::<std::string::String>()
    }
}
