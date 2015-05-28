#![deny(
    unused_allocation,
    unused_attributes,
    unused_features,
    unused_import_braces,
    unused_parens,
    unused_must_use,

    bad_style,
    unused
)]

extern crate sfml;
extern crate rand;
mod scenario;

use sfml::window::{self, ContextSettings, VideoMode, event};
use sfml::system::Vector2i;
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use sfml::window::mouse::MouseButton::MouseLeft;
use scenario::Scenario;

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(1000, 1000, 32),
                                       "Simuate Traffic",
                                       window::Close,
                                       &ContextSettings::default())
                         .expect("Cannot create a new Render Window.");
    let mut view = window.get_default_view();

    let mut scenario = Scenario::new()
        .with_cars(1000, "Sedan");

    let mut dragging = false;
    let mut last_x = 0;
    let mut last_y = 0;

    window.set_framerate_limit(60);
    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::MouseWheelMoved { delta, x: _, y: _ } => {
                    if delta > 0 {
                        view.zoom(0.9);
                    } else {
                        view.zoom(1.1);
                    }
                },
                event::MouseButtonPressed { button: MouseLeft, x, y } => {
                    dragging = true;
                    last_x = x;
                    last_y = y;
                },
                event::MouseButtonReleased { button: MouseLeft, x: _, y: _ } => dragging = false,
                event::MouseMoved { x, y } => {
                    if dragging {
                        let last = window.map_pixel_to_coords_current_view(&Vector2i::new(last_x, last_y));
                        let current = window.map_pixel_to_coords_current_view(&Vector2i::new(x, y));
                        let delta = last - current;

                        view.move_(&delta);

                        last_x = x;
                        last_y = y;
                    }
                },
                _             => {/* do nothing */}
            }
        }

        window.set_view(&view);

        window.clear(&Color::new_rgb(0, 0, 0));
        scenario.tick();
        window.draw(&scenario);
        window.display()
    }
}
