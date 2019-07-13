#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn another_hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("another hello from rust"))
}

fn render(mut cx: FunctionContext) -> JsResult<JsBuffer> {
  let data = cx.argument::<JsBuffer>(0)?;
  let w = cx.number(255);
  let b = cx.number(0);
  for x in 0..50 {
    for y in 0..50 {
      let index = (y * 50 + x) * 4;
      data.set(&mut cx, index, w)?;
      data.set(&mut cx, index + 1, b)?;
      data.set(&mut cx, index + 2, b)?;
      data.set(&mut cx, index + 3, w)?;
    }
  }
  Ok(data)
}

register_module!(mut m, {
  m.export_function("hello", hello)?;
  m.export_function("another_hello", another_hello)?;
  m.export_function("render", render)?;
  Ok(())
});