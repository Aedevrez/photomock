#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use image::io::Reader as ImageReader;

#[derive(Routable, Clone)]
enum Route {
        #[route("/")]
        Home {},
        #[route("/blog")]
        Blog {},
        #[route("/devs")]
        Devs {}
}


fn main() {
    dioxus_desktop::launch(App);
}

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
                oninput: move |evt| {filename.set(evt.value.clone());},
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
    let img_name = name.split(".").collect::<Vec<&str>>();
    let name = img_name[0];
    let ext = img_name[1];
    img.save(format!("{name}_grayscaled.{ext}")).unwrap();
}

async fn blurrer(name: String) -> () {
    let img = ImageReader::open(name.clone()).unwrap().decode().unwrap().blur(5.0);
    let img_name = name.split(".").collect::<Vec<&str>>();
    let name = img_name[0];
    let ext = img_name[1];
    img.save(format!("{name}_blurred.{ext}")).unwrap();
}

async fn sharpener(name: String) -> () {
    let img = ImageReader::open(name.clone()).unwrap().decode().unwrap().unsharpen(5.0, 5);
    let img_name = name.split(".").collect::<Vec<&str>>();
    let name = img_name[0];
    let ext = img_name[1];
    img.save(format!("{name}_sharpened.{ext}")).unwrap();
}