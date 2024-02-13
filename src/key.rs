use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key<'a> {
    Bool(bool),
    I64(i64),
    String(Box<Cow<'a, str>>)
}

convert!(enum Key<'a>, variant Key::Bool: type bool);
convert!(enum Key<'a>, variant Key::I64: type i64);
convert!(enum Key<'a>, variant Key::String: type Box<Cow<'a, str>>);

convert!(enum Key<'a>, type i64: from i8, i16, i32, u8, u16, u32);
convert!(enum Key<'a>, type i64: from_as u64, usize, isize);
convert!(enum Key<'a>, type Box<Cow<'a, str>>: from Cow<'a, str>);
convert!(enum Key<'a>, type Box<Cow<'a, str>>: ref Cow<'a, str>);
convert!(enum Key<'a>: cow str, String);

impl<'a> AsRef<Key<'a>> for Key<'a> {
    fn as_ref(&self) -> &Key<'a> {
        self
    }
}

impl<'a> AsMut<Key<'a>> for Key<'a> {
    fn as_mut(&mut self) -> &mut Key<'a> {
        self
    }
}
