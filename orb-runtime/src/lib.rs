use wasmtime::*;

pub fn get_wasm_string() -> wasmtime::Result<String> {
    let mut store = Store::<()>::default();

    let module = Module::from_file(
        store.engine(),
        "../target/wasm32-unknown-unknown/debug/wasm_test.wasm",
    )?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let get_func = instance.get_func(&mut store, "tacocat").unwrap();

    get_func.call(&mut store, &[900.into()], &mut [])?;

    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let mut info_buf = [0u8; 8];
    memory.read(&mut store, 900, &mut info_buf)?;
    let addr = i32::from_le_bytes(info_buf[0..4].try_into().unwrap());
    let len = i32::from_le_bytes(info_buf[4..8].try_into().unwrap());

    let mut buffer = vec![0u8; len as usize];
    memory.read(&mut store, addr as usize, &mut buffer)?;

    let message = String::from_utf8(buffer).unwrap();

    Ok(message)
}
