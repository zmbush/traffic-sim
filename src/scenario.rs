use std::fmt::{self, Debug};
use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, CircleShape, Color};
use sfml::traits::Drawable;
use rand::{thread_rng, Rng};
use std::f32;

trait Driver : Debug {
    fn next_destination(&mut self) -> Vector2f;
}

#[derive(Debug)]
struct DriveHomeDriver;

impl DriveHomeDriver {
    fn new() -> DriveHomeDriver {
        DriveHomeDriver
    }
}

impl Driver for DriveHomeDriver {
    fn next_destination(&mut self) -> Vector2f {
        Vector2f::new(
          thread_rng().gen_range(0., 1000.),
          thread_rng().gen_range(0., 1000.)
        )
    }
}

#[derive(Debug)]
struct Car {
    location: Vector2f,
    destination: Vector2f,

    heading: f32,
    name: String,
    color: Color,

    driver: Box<Driver>
}

impl Car {
    fn new<S, D>(color: Color, name: S, driver: Box<D>) -> Car where
            S: ToString,
            D: Driver+'static
    {
        Car {
            location: Vector2f::new(
              thread_rng().gen_range(0., 1000.),
              thread_rng().gen_range(0., 1000.)
            ),

            destination: Vector2f::new(
              thread_rng().gen_range(0., 1000.),
              thread_rng().gen_range(0., 1000.)
            ),

            heading: 0.0,
            name: name.to_string(),
            color: color,

            driver: driver
        }
    }

    fn tick(&mut self, dest: Vector2f) {
        self.destination = dest;

        let delta = self.location - self.destination;

        let target_heading = (delta.x.atan2(-delta.y) / f32::consts::PI) * 180.;

        for _ in 0..3 {
            let heading_delta = ((target_heading - self.heading) + 180.) % 360. - 180.;

            if heading_delta > 0. {
                // println!("{} {} => {}", 5., heading_delta, f32::min(5., heading_delta));
                self.heading += f32::min(1., heading_delta);
            } else {
                self.heading += f32::max(-1., heading_delta);
            }

            self.heading %= 360.;

            let th = (self.heading / 180.0f32)*f32::consts::PI;
            self.location.y += th.cos();
            self.location.x += -th.sin();
            /*
            let dist = self.location - self.destination;
            if (dist.x.powf(2.) + dist.y.powf(2.)).sqrt() < 2. {
                self.destination = self.driver.next_destination();
            }
            */
        }
    }

    fn behind(&self, dist: f32) -> Vector2f {
        let th = (self.heading / 180.) * f32::consts::PI;
        let (x, y) = (-th.sin(), th.cos());

        Vector2f::new(self.location.x + (x*-dist), self.location.y + (y*-dist))
    }
}

impl Drawable for Car {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        let mut shape = RectangleShape::new_init(&Vector2f::new(10., 20.)).expect("Error, cannot draw car!!!");
        shape.set_position(&self.location);
        shape.set_origin(&Vector2f::new(5., 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = RectangleShape::new_init(&Vector2f::new(1., 50.)).expect("Error, cannot draw line!!!");
        shape.set_position(&self.location);
        shape.set_origin(&Vector2f::new(0.5, 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = CircleShape::new().expect("Error, cannot create ball.");
        shape.set_radius(5.);
        shape.set_fill_color(&self.color);
        shape.set_position(&self.destination);
        shape.set_origin(&Vector2f::new(5., 5.));
        //target.draw(&shape);
    }
}

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
            self.cars.push(Car::new(c, format!("{} {}", name, i), Box::new(DriveHomeDriver::new())));
        }
        self
    }

    pub fn tick(&mut self) {
        for _ in 0..30 {
            for i in 0..self.cars.len() {
                let dest = self.cars[(i+1)%(self.cars.len())].behind(30.);
                self.cars[i].tick(dest);
            }
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
