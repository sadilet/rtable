use std::borrow::Cow;
use std::cmp;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::Key;

type ListD<'a> = Vec<Value<'a>>;
type MapD<'a> = HashMap<Key<'a>, Value<'a>>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(Box<Cow<'a, str>>),
    List(Box<Cow<'a, ListD<'a>>>),
    Map(Box<Cow<'a, MapD<'a>>>),
}

convert!(enum Value<'a>, variant Value::Bool: type bool);
convert!(enum Value<'a>, variant Value::I64: type i64);
convert!(enum Value<'a>, variant Value::F64: type f64);
convert!(enum Value<'a>, variant Value::String: type Box<Cow<'a, str>>);
convert!(enum Value<'a>, variant Value::List: type Box<Cow<'a, ListD<'a>>>);
convert!(enum Value<'a>, variant Value::Map: type Box<Cow<'a, MapD<'a>>>);

convert!(enum Value<'a>, type i64: from i8, i16, i32, u8, u16, u32);
convert!(enum Value<'a>, type i64: from_as u64, isize, usize);
convert!(enum Value<'a>, type f64: from f32);
convert!(enum Value<'a>, type Box<Cow<'a, str>>: from Cow<'a, str>);
convert!(enum Value<'a>, type Box<Cow<'a, str>>: ref Cow<'a, str>);
convert!(enum Value<'a>, type Box<Cow<'a, ListD<'a>>>: from Cow<'a, ListD<'a>>);
convert!(enum Value<'a>, type Box<Cow<'a, ListD<'a>>>: ref Cow<'a, ListD<'a>>);
convert!(enum Value<'a>, type Box<Cow<'a, MapD<'a>>>: from Cow<'a, MapD<'a>>);
convert!(enum Value<'a>, type Box<Cow<'a, MapD<'a>>>: ref Cow<'a, MapD<'a>>);
convert!(enum Value<'a>: cow str, String);
convert!(enum Value<'a>: cow ListD<'a>, ListD<'a>);
convert!(enum Value<'a>: cow MapD<'a>, MapD<'a>);

impl<'a> Default for Value<'a> {
    fn default() -> Self {
        Value::Null
    }
}

impl<'a> AsRef<Value<'a>> for Value<'a> {
    fn as_ref(&self) -> &Value<'a> {
        self
    }
}

impl<'a> AsMut<Value<'a>> for Value<'a> {
    fn as_mut(&mut self) -> &mut Value<'a> {
        self
    }
}

impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Value<'a>) -> Option<cmp::Ordering> {
        use Value::*;

        match (self, other) {
            (Null, Null) => Some(cmp::Ordering::Equal),
            (Bool(x), Bool(y)) => x.partial_cmp(y),
            (I64(x), I64(y)) => x.partial_cmp(y),
            (F64(x), F64(y)) => x.partial_cmp(y),
            (String(x), String(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}