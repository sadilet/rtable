extern crate cfg_if;
mod debug;
#[macro_use]
mod macros;
mod key;
mod value;

pub mod util;

use std::borrow::Cow;
pub use value::Value;
pub use key::Key;

use wasm_bindgen::prelude::*;
use std::collections::{HashMap, VecDeque};
use serde::Serialize;
use serde_wasm_bindgen::{Serializer};
use crate::value::MapD;

#[wasm_bindgen]
pub struct Table {
    rows: VecDeque<Box<MapD<'static>>>,
    cols: Vec<String>
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(constructor)]
    pub fn new(columns: Option<Vec<String>>) -> Table {
        let mut data = VecDeque::with_capacity(1100000);
        for i in 1..1000000 {
            data.push_back(Box::new(
                HashMap::from(
                    [
                        (Key::from("Mercury"), Value::from(1 + i)),
                        (Key::from("Venus"), Value::from(8 + i)),
                        (Key::from("Earth"), Value::from(9 + i)),
                        (Key::from("Mars"), Value::from(10 + i)),
                    ]
                )
            ));
        }
        let cols = if columns.is_none() {
            data[0].keys().into_iter().map(|x| match x {Key::String(v) => v.to_string()}).collect()
            // data[0].keys().map(|x| match x {Key::String(v) => v})
        } else {
            columns.unwrap()
        };
        Table{ rows: data, cols: cols }
    }

    pub fn cols(&self) -> JsValue {
        self.cols.serialize(&Serializer::json_compatible()).unwrap()
    }

    pub fn insert_col(&mut self, name: String) {
        self.cols.push(name)
    }

    pub fn insert_row(&mut self) {
        self.rows.push_back(Box::new(MapD::from(HashMap::new())))
    }

    pub fn rows(&self, start: usize, end: usize) -> JsValue {
        debug::set_panic_hook();

        let chunk = self.rows.range(start..end).collect::<Vec<_>>();
        chunk.serialize(&Serializer::json_compatible()).unwrap()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
