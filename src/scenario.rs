use std::fmt;
use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, CircleShape, Color};
use sfml::traits::Drawable;
use rand::{thread_rng, Rng};
use std::f32;

#[derive(Clone, Debug)]
struct Car {
    x: f32,
    y: f32,

    dest_x: f32,
    dest_y: f32,

    heading: f32,
    name: String,
    color: Color
}

impl Car {
    fn new<S>(color: Color, name: S) -> Car where S: ToString {
        Car {
            x: thread_rng().gen_range(0., 1000.),
            y: thread_rng().gen_range(0., 1000.),

            dest_x: thread_rng().gen_range(0., 1000.),
            dest_y: thread_rng().gen_range(0., 1000.),

            heading: 0.0,
            name: name.to_string(),
            color: color
        }
    }

    fn tick(&mut self) {
        let x_delta = self.x - self.dest_x;
        let y_delta = self.dest_y - self.y;
        let target_heading = (x_delta.atan2(y_delta) / f32::consts::PI) * 180.;
        let heading_delta = ((target_heading - self.heading) + 180.) % 360. - 180.;

        if heading_delta > 0. {
            self.heading += f32::min(5., heading_delta);
        } else {
            self.heading -= f32::max(5., heading_delta);
        }

        let th = (self.heading / 180.0f32)*f32::consts::PI;
        self.y += th.cos();
        self.x += -th.sin();
    }
}

impl Drawable for Car {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        let mut shape = RectangleShape::new_init(&Vector2f::new(10., 20.)).expect("Error, cannot draw car!!!");
        shape.set_position(&Vector2f::new(self.x, self.y));
        shape.set_origin(&Vector2f::new(5., 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = RectangleShape::new_init(&Vector2f::new(1., 50.)).expect("Error, cannot draw line!!!");
        shape.set_position(&Vector2f::new(self.x, self.y));
        shape.set_origin(&Vector2f::new(0.5, 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = CircleShape::new().expect("Error, cannot create ball.");
        shape.set_radius(5.);
        shape.set_fill_color(&self.color);
        shape.set_position(&Vector2f::new(self.dest_x, self.dest_y));
        shape.set_origin(&Vector2f::new(5., 5.));
        target.draw(&shape);
    }
}

#[derive(Clone)]
pub struct Scenario {
    cars: Vec<Car>
}

impl Scenario {
    pub fn new() -> Scenario {
        Scenario {
            cars: Vec::new()
        }
    }

    pub fn with_cars<S>(mut self, n: i64, name: S) -> Scenario where S: fmt::Display {
        for i in 0..n {
            let mut rng = thread_rng();
            let c = Color::new_rgb(rng.gen(), rng.gen(), rng.gen());
            self.cars.push(Car::new(c, format!("{} {}", name, i)));
        }
        self
    }

    pub fn tick(&mut self) {
        for car in self.cars.iter_mut() {
            car.tick();
        }
    }
}

impl Drawable for Scenario {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        for car in self.cars.iter() {
            target.draw(car);
        }
    }
}
