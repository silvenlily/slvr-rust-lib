use crate::tauri_wasm::{Message, MsgSafe};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "tauri_wasm_frontend")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_args(cmd: &str, args: JsValue) -> JsValue;
}

pub trait TauriCommand<Result: TauriResult>: MsgSafe {
    const COMMAND_NAME: &'static str;

    #[cfg(feature = "tauri_wasm_frontend")]
    fn send(self) -> impl Future<Output = Result> {
        async move {
            Result::from_js(invoke_args(Self::COMMAND_NAME, self.to_js()).await)
        }
    }

    #[cfg(feature = "tauri_wasm_backend")]
    fn handle(self) -> impl Future<Output = Result>;
}

pub trait TauriResult: MsgSafe {}
