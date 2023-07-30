use maud::html;

#[no_mangle]
pub fn tacocat() -> String  {
    let markup = html! {
        h1 { "Hello, world!" }
    };

    markup.0
}
