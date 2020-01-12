extern crate neon;
extern crate blake3;

use neon::prelude::*;

fn hash(mut cx: FunctionContext) -> JsResult<JsString> {
	let arg = cx.argument::<JsBuffer>(0)?;
    let input = cx.borrow(&arg, |data| data.as_slice::<u8>());

    let mut hasher = blake3::Hasher::new();
    hasher.update(input);
    let result = hasher.finalize();
    let output = cx.string(result.to_hex());

    Ok(output)
}

pub struct HashCtx {
    hasher: blake3::Hasher
}

declare_types! {
    pub class JsHashCtx for HashCtx {
        init(_) {
            Ok(HashCtx {
                hasher: blake3::Hasher::new()
            })
        }

        method update(mut cx) {
            let arg = cx.argument::<JsBuffer>(0)?;
            let input = cx.borrow(&arg, |data| data.as_slice::<u8>());

            let mut this = cx.this();
            let guard = cx.lock();
            this.borrow_mut(&guard).hasher.update(input);

            Ok(cx.undefined().upcast())
        }

        method digest(mut cx) {
            let mut this = cx.this();
            let guard = cx.lock();

            let result = this.borrow_mut(&guard).hasher.finalize();
            let output = cx.string(result.to_hex());

            Ok(output.upcast())
        }
    }
}

register_module!(mut m, {
    m.export_function("hash", hash)?;
    m.export_class::<JsHashCtx>("HashCtx")
});
