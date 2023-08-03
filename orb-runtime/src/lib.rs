use std::collections::HashMap;

use wasmtime::*;

#[derive(Clone, Copy, Debug)]
enum StringFormat {
    U8,
    U16,
}

#[derive(Debug)]
struct StoreData {
    output: i32,
    size: i32,
    format: StringFormat,
}

impl Default for StoreData {
    fn default() -> Self {
        Self {
            output: 0,
            size: 0,
            format: StringFormat::U8,
        }
    }
}

#[derive(Clone)]
pub struct Runtime {
    pub engine: Engine,
    linker: Linker<StoreData>,
    modules: HashMap<String, Module>,
}

pub static MAGIC_STRING: &'static str = "orb";
impl Runtime {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap(
                "host",
                "check_magic_string",
                |mut caller: Caller<'_, StoreData>, string_loc: i32| {
                    let mut buffer = vec![0u8; MAGIC_STRING.len() * 2];
                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    memory
                        .read(&caller, string_loc as usize, &mut buffer)
                        .unwrap();

                    if String::from_utf8(buffer[0..MAGIC_STRING.len()].to_vec()).unwrap()
                        == MAGIC_STRING
                    {
                        caller.data_mut().format = StringFormat::U8;
                        return;
                    }

                    if String::from_utf16(
                        buffer
                            .chunks(2)
                            .map(|x| u16::from_le_bytes([x[0], x[1]]))
                            .collect::<Vec<u16>>().as_slice(),
                    )
                    .unwrap()
                        == MAGIC_STRING
                    {
                        caller.data_mut().format = StringFormat::U16;
                        return;
                    }
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "host",
                "get",
                |mut caller: Caller<'_, StoreData>, addr: i32, size: i32| {
                    let data = caller.data_mut();

                    data.output = addr;
                    data.size = size;
                },
            )
            .unwrap();

        linker
            .func_wrap("env", "abort", |_a: i32, _b: i32, _c: i32, _d: i32| todo!())
            .unwrap();

        Self { engine, linker, modules: HashMap::new() }
    }

    pub fn get_modules(&self) -> &HashMap<String, Module> {
        &self.modules
    }

    pub fn add_module(&mut self, name: &str, bytes: &[u8]) -> wasmtime::Result<()> {
        let module = Module::from_binary(self.get_engine(), bytes).unwrap();
        self.modules.insert(name.to_string(), module.clone());

        let nuuid = uuid::Uuid::new_v4().to_string();
        println!("nuuid: {}", nuuid);
        let result = self.modules.insert(nuuid, module);

        println!("result: {:?}", self.modules.len());

        match result {
            Some(_) => Err(wasmtime::Error::msg("asdf")),
            None => Ok(()),
        }
    }

    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }

    pub fn get_wasm_string(&self, module_name: String) -> wasmtime::Result<String> {
        println!("module_name: {}", module_name);
        println!("modules: {:?}", self.modules.len());
        let module = self.modules.get(&module_name).unwrap();
        let mut store = Store::new(self.get_engine(), StoreData::default());
        let instance = self.linker.instantiate(&mut store, module)?;

        let get_func = instance.get_func(&mut store, "tacocat").unwrap();
        get_func.call(&mut store, &[], &mut [])?;

        println!("output: {:?}", store.data());

        let data = store.data();
        let addy = data.output;
        let size = data.size;
        let format = data.format;

        match format {
            StringFormat::U8 => {
                let memory = instance.get_memory(&mut store, "memory").unwrap();
                let mut buffer = vec![0u8; size as usize];

                memory.read(&mut store, addy as usize, &mut buffer).unwrap();

                let string = std::str::from_utf8(buffer.as_slice()).unwrap();

                Ok(string.to_string())
            }
            StringFormat::U16 => {
                let memory = instance.get_memory(&mut store, "memory").unwrap();
                let mut buffer = vec![0u8; (size * 2 - 1) as usize];

                memory.read(&mut store, addy as usize, &mut buffer).unwrap();

                let string = std::str::from_utf8(buffer.as_slice()).unwrap();

                Ok(string.to_string())
            }
        }
    }
}
