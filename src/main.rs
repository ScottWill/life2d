use crate::world::Model;
use nannou::{App, Frame, prelude::Update};

mod grid;
mod world;

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
    app.new_window()
        .event(event_fn)
        .resizable(false)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();
    Model::new(WIDTH, HEIGHT, SCALE)
}

fn event_fn(app: &App, model: &mut Model, event: nannou::event::WindowEvent) {
    model.handle_event(app, event);
}

fn update(_: &App, model: &mut Model, _: Update) {
    model.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.view(app, &draw);
    draw.to_frame(app, &frame).unwrap();
}