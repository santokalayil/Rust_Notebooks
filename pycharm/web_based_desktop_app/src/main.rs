// cargo add web-view // for installtion

use web_view::*;

// fn get_page(page_name: &str) -> &'static str {  // fn get_page(page_name: &str)
//
//     if page_name == "index" {
//         println!("Opening INDEX page! {}", page_name);
//         return "<html><body><h1>NEW PAGE IN RUST</h1></body></html>";
//     } else {
//         "empty page"  // if no semicolon compiler finds it as return value
//     }
//
//
// }


use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    page_name: &'a str, // the field name should match the variable name
                   // in your template
}




fn main() {
    let hello = HelloTemplate { page_name: "MY DYNAMIC VARIABLE" }; // instantiate your struct
    // println!("{}", hello.render().unwrap()); // then render it.

    // this for using get page function if needed...
    // let html_content = get_page("index" as &str); // &String::from("index")
    let html_content = hello.render().unwrap();

    web_view::builder()
        .title("Ecclessiastica")
        .content(Content::Html(html_content))
        .size(560, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}