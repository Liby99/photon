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
  for x in 0..50 {
    for y in 0..50 {
      let index = (y * 50 + x) * 4;
      let r = cx.number(x * 5);
      let g = cx.number(y * 5);
      data.set(&mut cx, index, r)?;
      data.set(&mut cx, index + 1, g)?;
      data.set(&mut cx, index + 2, w)?;
      data.set(&mut cx, index + 3, w)?;
    }
  }
  Ok(data)
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("another_hello", another_hello)?;
  cx.export_function("render", render)?;
  Ok(())
});