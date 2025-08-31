
#[cfg(feature = "tauri_wasm_frontend")]
pub mod command;

use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

pub trait MsgSafe: Sized + Send + Sync + Debug + Serialize + DeserializeOwned {}

pub trait Message {
    fn to_js(&self) -> JsValue;

    fn from_js<'a>(from: JsValue) -> Self;
}

impl<T: MsgSafe> Message for T {
    fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).expect("Could not convert rust object to js object")
    }

    fn from_js(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value::<Self>(value)
            .expect("Could not convert js object to rust object")
    }
}


