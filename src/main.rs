use clap::Parser;
use crate::{config::Args, world::Model};
use nannou::{event::WindowEvent, prelude::Update, App, Frame};
use std::time::Instant;

mod config;
mod grid;
mod world;

const APP_NAME: &'static str = "2D Life";

// trait EventHandler {
//     fn handle_event(&mut self, app: &App, event: &WindowEvent);
// }

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    let mut builder = app.new_window()
        .event(event_fn)
        .resizable(false);
    
    let mut args = Args::parse();
    assert!(args.resolution > 0, "`resolution` must be greater than zero");
    assert!(args.speed > 0, "`speed` must be greater than zero");

    builder = if args.fullscreen {
        builder.fullscreen()
    } else {
        builder.size(args.width, args.height)
    };
        
    let id = builder.build().unwrap();
    let rect = app.window(id).unwrap().rect();
    args.height = rect.h() as u32;
    args.width = rect.w() as u32;

    Model::new(&args)
}

fn event_fn(app: &App, model: &mut Model, event: WindowEvent) {
    model.handle_event(app, event);
}

fn update(app: &App, model: &mut Model, _: Update) {
    let now = Instant::now();
    app.main_window().set_title(&title!(model.title_meta()));
    model.step();
    if model.debug {
        println!("update: {:?}", now.elapsed());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let now = Instant::now();
    let draw = app.draw();
    model.view(app, &draw);
    draw.to_frame(app, &frame).unwrap();
    if model.debug {
        println!("view: {:?}, {} fps", now.elapsed(), app.fps());
    }
}

#[macro_export]
macro_rules! title {
    ($x:expr) => {
        {
            let (a, b) = $x;
            format!("{APP_NAME} - {} - {}", a, b)
        }
    }
}