#![windows_subsystem = "windows"]  // to hide console window

use fltk::{app::*, button::*, frame::*, window::*, image::*};  //

// use these if need to embed image
#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;
// --------------------------------|





fn main() {
    let app = App::default();
    let mut win = Window::new(100, 100, 400, 300, "Santo's App",);

    let img = Assets::get("app_icon.png").unwrap(); // these 2 lines to embed if not use line after next line only
    let image = PngImage::from_data(&img).unwrap();

    // let image = PngImage::load(&std::path::Path::new("assets/app_icon.png")).unwrap();

    win.set_icon(Some(image));
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    win.end();
    win.show();
    but.set_callback(move || frame.set_label("You clicked the Button!"));
    app.run().unwrap();
}