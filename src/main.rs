#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use image::io::Reader as ImageReader;

// All of our routes will be a variant of this Route enum
#[derive(Routable, Clone)]
enum Route {
        // if the current location is "/home", render the Home component
        #[route("/")]
        Home {},
        // if the current location is "/blog", render the Blog component
        #[route("/blog")]
        Blog {},
        #[route("/devs")]
        Devs {}
}


fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Route> {}
    })
}

fn Home(cx: Scope) -> Element {
    let my_style = "body { margin: 0 }";
    let link_style = "a { width: 29vw; height: 10vh; border: solid 5px #F9FAF8; border-radius: 15px; text-decoration: none; color: #F9FAF8; display: flex; justify-content: center; align-items: center; font-size: 3vw; font-weight: 700}";
    cx.render(rsx! {
        style { my_style }
        main {
            margin: "0px",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            background_color: "#1F2937",
            height: "100vh",
            width: "100vw",
            gap: "9vh",
            div {
                color: "#F9FAF8",
                font_size: "7vw",
                font_weight: "900",
                "PHOTOMOCK"
            }
            Link {
                to: Route::Blog {},
                style{ link_style },
                "Start Editing!"
            }
            Link {
                to: Route::Devs {},
                style{ link_style },
                "Developers!"
            }
        }
    })
}

fn Blog(cx: Scope) -> Element {
    let my_style = "body { margin: 0 }";
    let filename = use_state(cx, || "lenna.png".to_string());
    let link_style = "a { width: 29vw; height: 10vh; border: solid 5px #F9FAF8; border-radius: 15px; text-decoration: none; color: #F9FAF8; display: flex; justify-content: center; align-items: center; font-size: 3vw; font-weight: 700}";
    cx.render(rsx! {
        style { my_style }
        main {
            margin: "0px",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            height: "100vh",
            width: "100vw",
            background_color: "#1F2937",
            gap: "9vh",
            input {
                width: "29vw",
                height: "10vh",
                border: "solid 5px #F9FAF8",
                border_radius: "15px",
                text_decoration: "none",
                color: "#F9FAF8",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                font_size: "5vw",
                font_weight: "700",
                background_color: "#1F2937",
                value: "lenna.png",
                oninput: move |evt| {filename.set(evt.value.clone()); println!("{filename}");},
            }
            button {
                width: "29vw",
                height: "10vh",
                border: "solid 5px #F9FAF8",
                border_radius: "15px",
                text_decoration: "none",
                color: "#F9FAF8",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                font_size: "3vw",
                font_weight: "700",
                background_color: "#1F2937",
                onclick: move |_| {grayscaler(filename.to_string())},
                "Grayscale!"
            }
            button {
                width: "29vw",
                height: "10vh",
                border: "solid 5px #F9FAF8",
                border_radius: "15px",
                text_decoration: "none",
                color: "#F9FAF8",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                font_size: "3vw",
                font_weight: "700",
                background_color: "#1F2937",
                onclick: move |_| {blurrer(filename.to_string())},
                "Blur!"
            }
            button {
                width: "29vw",
                height: "10vh",
                border: "solid 5px #F9FAF8",
                border_radius: "15px",
                text_decoration: "none",
                color: "#F9FAF8",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                font_size: "3vw",
                font_weight: "700",
                background_color: "#1F2937",
                onclick: move |_| {sharpener(filename.to_string())},
                "Sharpen!"
            }
            Link {
                    style{ link_style },
                    to: Route::Home {},
                    "Go home!"
            }
        }
    })
}

fn Devs(cx: Scope) -> Element {
    let my_style = "body { margin: 0 }";
    let link_style = "a { width: 29vw; height: 10vh; border: solid 5px #F9FAF8; border-radius: 15px; text-decoration: none; color: #F9FAF8; display: flex; justify-content: center; align-items: center; font-size: 3vw; font-weight: 700}";
    cx.render(rsx! {
        style { my_style }
        main {
            margin: "0px",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            height: "100vh",
            width: "100vw",
            background_color: "#1F2937",
            gap: "7vh",
            div {
                color: "#F9FAF8",
                font_size: "5vw",
                font_weight: "700",
                "Adem Efe Devrez"
            }
            div {
                color: "#F9FAF8",
                font_size: "5vw",
                font_weight: "700",
                "Mustafa Taner Turan"
            }
            div {
                width: "29vw",
                height: "10vh",
                border: "solid 5px #F9FAF8",
                border_radius: "15px",
                text_decoration: "none",
                color: "#F9FAF8",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                font_size: "3vw",
                font_weight: "700",
                background_color: "#1F2937",
                Link {
                    style{ link_style },
                    to: Route::Home {},
                    "Go home!"
                }
            }
        }
    })
}

async fn grayscaler(name: String) -> () {
    let img = ImageReader::open(name.clone()).unwrap().decode().unwrap().grayscale();
    let name = name.split(".").collect::<Vec<&str>>()[0];
    img.save(format!("{name}_grayscaled.png")).unwrap();
}

async fn blurrer(name: String) -> () {
    let img = ImageReader::open(name.clone()).unwrap().decode().unwrap().blur(5.0);
    let name = name.split(".").collect::<Vec<&str>>()[0];
    img.save(format!("{name}_blurred.png")).unwrap();
}

async fn sharpener(name: String) -> () {
    let img = ImageReader::open(name.clone()).unwrap().decode().unwrap().unsharpen(5.0, 5);
    let name = name.split(".").collect::<Vec<&str>>()[0];
    img.save(format!("{name}_sharpened.png")).unwrap();
}