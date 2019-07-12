#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn another_hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("another hello from rust"))
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("another_hello", another_hello)
});