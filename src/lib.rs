extern crate cfg_if;
mod debug;
#[macro_use]
mod macros;
mod key;
mod value;

pub mod util;
pub use value::Value;
pub use key::Key;

use wasm_bindgen::prelude::*;
use std::collections::{HashMap, VecDeque};
use serde::Serialize;
use serde_wasm_bindgen::{Serializer};

#[wasm_bindgen]
pub struct Table {
    data: VecDeque<Box<HashMap<Key<'static>, Value<'static>>>>,
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Table {
        let mut data = VecDeque::with_capacity(110000);
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
        Table{ data: data }
    }

    pub fn rows(&self, start: usize, end: usize) -> JsValue {
        debug::set_panic_hook();

        let chunk = self.data.range(start..end).collect::<Vec<_>>();
        chunk.serialize(&Serializer::json_compatible()).unwrap()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
