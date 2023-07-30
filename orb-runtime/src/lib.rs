use wasmtime::*;

pub struct Runtime {
    pub engine: Engine,
    linker: Linker<()>
}

impl Runtime {
    pub fn new() -> Self {
        let engine = Engine::default();
        let linker = Linker::new(&engine);

        Self { engine, linker }
    }

    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }

    pub fn get_wasm_string(&self, module: &Module) -> wasmtime::Result<String> {
        let mut store = Store::new(self.get_engine(), ());

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
}

pub fn get_module(engine: &Engine, path: &str) -> Module {
    Module::from_file(
        engine,
        path,
    ).unwrap()
}
