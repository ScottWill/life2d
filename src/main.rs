use std::time::Instant;

use clap::Parser;
use crate::world::Model;
use nannou::{App, Frame, prelude::Update, event::WindowEvent};

mod grid;
mod world;

const APP_NAME: &'static str = "2D Life";

/// 2D Life simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output debug information
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    /// Overrides both width/height if true
    #[arg(short, long, default_value_t = false)]
    fullscreen: bool,
    /// Window Width
    #[arg(short, long, default_value_t = 1200)]
    width: u32,
    /// Window Height
    #[arg(short, long, default_value_t = 900)]
    height: u32,
    /// Grid Scale
    #[arg(short, long, default_value_t = 2)]
    scale: u32,
}

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
    
    let args = Args::parse();
    assert!(args.scale > 0, "`scale` must be greater than zero");

    builder = if args.fullscreen {
        builder.fullscreen()
    } else {
        builder.size(args.width, args.height)
    };
        
    let id = builder.build().unwrap();
    let rect = app.window(id).unwrap().rect();
    Model::new(rect.w() as u32, rect.h() as u32, args.scale, args.debug)
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
        println!("view: {:?}", now.elapsed());
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