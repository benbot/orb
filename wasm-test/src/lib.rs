use maud::html;

#[link(wasm_import_module = "host")]
extern "C" {
    fn check_magic_string(addr: i32);
    fn get(addr: i32, size: i32);
}

#[no_mangle]
pub fn tacocat() {
    let markup = html! {
        h1 { "Hello, world!" }
    };

    unsafe {
        check_magic_string("orb".as_ptr() as i32);
        get(markup.0.as_ptr() as i32, markup.0.len() as i32);
    };
}
