use deno_core::v8;
use deno_core::BufVec;
use deno_core::JsRuntime;
use deno_core::Op;
use deno_core::OpState;
use deno_core::RuntimeOptions;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::time::SystemTime;

const SCRIPT: &str = r#"
  "use strict";
  const { core } = Deno;
  const { jsonOpSync } = core;
  const { op_pummel } = core.ops();

  const args = {};
  for (let i = 0; i < 42; i++) args["k" + i] = i;
  for (let i = 0; i < 1e6; i++) jsonOpSync("op_pummel", args);
"#;

fn main() {
  // So I can pass --perf_basic_prof to V8 when needed.
  for arg in env::args().skip(1) {
    v8::V8::set_flags_from_string(&arg);
  }

  let mut js = JsRuntime::new(RuntimeOptions::default());
  js.register_op("op_pummel", op_pummel);

  let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();

  let fut = async move {
    let start = SystemTime::now();
    js.execute("script.js", SCRIPT).unwrap();
    let elapsed = start.elapsed().unwrap();
    println!("{:?}", elapsed);
  };

  rt.block_on(fut)
}

fn op_pummel(_: Rc<RefCell<OpState>>, _: BufVec) -> Op {
  Op::Sync(br#"{"ok":true}"#[..].into())
}
