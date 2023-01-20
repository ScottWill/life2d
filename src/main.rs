use crate::world::Model;
use nannou::{App, Frame, prelude::Update};

mod grid;
mod world;

const APP_NAME: &'static str = "2D Life";
const HEIGHT: u32 = 900;
const WIDTH: u32 = 1200;
const SCALE: u32 = 2; // should be evenly divisible between both width and height

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    let model = Model::new(WIDTH, HEIGHT, SCALE);
    let meta = model.title_meta();
    app.new_window()
        .event(event_fn)
        .resizable(false)
        .title(title!(meta))
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();
    model
}

fn event_fn(app: &App, model: &mut Model, event: nannou::event::WindowEvent) {
    model.handle_event(app, event);
}

fn update(app: &App, model: &mut Model, _: Update) {
    let meta = model.title_meta();
    app.main_window().set_title(&title!(meta));
    model.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.view(app, &draw);
    draw.to_frame(app, &frame).unwrap();
}

#[macro_export]
macro_rules! title {
    ($x:expr) => {
        format!("{APP_NAME} - {} - {}", $x.0, $x.1)
    }
}