use std::sync::Arc;

use neon::prelude::*;
mod image;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn resize_by_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let source_path: Handle<JsString> = cx.argument(0)?;
    let target_path: Handle<JsString> = cx.argument(1)?;
    let nwidth: Handle<JsNumber> = cx.argument(2)?;
    let nheight: Handle<JsNumber> = cx.argument(3)?;
    let res = image::resize_by_path(&source_path.value(& mut cx), &target_path.value(&mut cx), nwidth.value(&mut cx) as u32, nheight.value(&mut cx) as u32);
    Ok(cx.string(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("resize_by_path", resize_by_path)?;
    Ok(())
}
