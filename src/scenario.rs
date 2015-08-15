use std::fmt::{self, Debug};
use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, CircleShape, Color};
use sfml::traits::Drawable;
use rand::{thread_rng, Rng};
use std::f32;

#[derive(Debug, Clone, Copy)]
struct Waypoint {
    location: Vector2f,
    speed: f32,
}

impl Waypoint {
    fn new(location: Vector2f, speed: f32) -> Waypoint {
        Waypoint {
            location: location,
            speed: speed
        }
    }

    fn new_2f(x: f32, y: f32, speed: f32) -> Waypoint {
        Waypoint::new(Vector2f::new(x, y), speed)
    }
}

trait Driver : Debug {
    fn next_destination(&mut self, car: &Car, scenario: &Vec<Car>) -> Waypoint;
}

#[derive(Debug)]
struct DriveHomeDriver;

impl DriveHomeDriver {
    fn new() -> DriveHomeDriver {
        DriveHomeDriver
    }
}

impl Driver for DriveHomeDriver {
    fn next_destination(&mut self, me: &Car, others: &Vec<Car>) -> Waypoint {
        let nearest_car = others.iter()
            .filter(|c| me.location != c.location)
            .map(|c| {
                let d = Color::new_rgb(
                    me.color.red - c.color.red,
                    me.color.green - c.color.green,
                    me.color.blue - c.color.blue);
                /*(Some(c),
                    (d.red as f32).powf(2.) +
                    (d.green as f32).powf(2.) +
                    (d.blue as f32).powf(2.))*/
                (Some(c), d.red as f32)
            })
            .fold((None, f32::INFINITY), |acc, item| {
                if item.1 < acc.1 && item.1 > 0. {
                    item
                } else {
                    acc
                }
            }).0;

        match nearest_car {
            Some(c) => Waypoint::new(c.behind(20.), 80.),
            None => Waypoint::new_2f(
              thread_rng().gen_range(0., 1000.),
              thread_rng().gen_range(0., 1000.),
              65.
            )
        }
    }
}

#[derive(Debug)]
struct Car {
    location: Vector2f,
    destination: Waypoint,

    wheel_angle: f32,
    heading: f32,
    speed: f32,
    name: String,
    turning_radius: f32,
    acceleration: f32,
    color: Color,

    driver: Option<Box<Driver>>
}

impl Car {
    fn new<S, D>(color: Color, name: S, driver: Box<D>) -> Car where
            S: ToString,
            D: Driver+'static
    {
        Car {
            location: Vector2f::new(
              thread_rng().gen_range(0., 1_000.),
              thread_rng().gen_range(0., 1_000.)
            ),

            destination: Waypoint::new(
                Vector2f::new(
                    thread_rng().gen_range(0., 1000.),
                    thread_rng().gen_range(0., 1000.)
                ), 80.
            ),

            turning_radius: 5.,
            acceleration: thread_rng().gen_range(1., 5.),

            wheel_angle: 0.0,
            heading: 0.0,
            speed: 0.0,
            name: name.to_string(),
            color: color,

            driver: Some(driver)
        }
    }

    fn shell_copy(&self) -> Car {
        Car {
            location: self.location,
            destination: self.destination,
            turning_radius: self.turning_radius,
            acceleration: self.acceleration,
            wheel_angle: self.wheel_angle,
            heading: self.heading,
            speed: self.speed,
            name: self.name.clone(),
            color: self.color,
            driver: None
        }
    }

    fn pixels_per_tick(&self) -> f32 {
        // (km / h) * (h / 3600 s) * (m*s/km*ms) * (10 px / m) * (17ms / tick)
        let pixels_per_meter = 10.;
        let millis_per_tick = 17.;
        let seconds_per_hour = 60.*60.;

        self.speed * pixels_per_meter * millis_per_tick / seconds_per_hour
    }

    // 1 tick = 1/60 second = 17ms
    // 10 pixel = 1m
    fn tick(&mut self, scene: &Vec<Car>) {
        for _ in 0..1 {
            let dist = self.location - self.destination.location;
            if dist.x.powf(2.) + dist.y.powf(2.) < self.destination.speed.powf(2.) {
                let shell = self.shell_copy();
                self.destination = match self.driver {
                    Some(ref mut driver) => {
                        driver.next_destination(&shell, scene)
                    },
                    None => panic!("Called tick with a shell car")
                }
            }

            let delta = self.location - self.destination.location;
            let target_heading = (delta.x.atan2(-delta.y) / f32::consts::PI) * 180.;

            let heading_delta = ((target_heading - self.heading - self.wheel_angle) + 180.) % 360. - 180.;

            if heading_delta > 0. {
                self.wheel_angle += f32::min(self.turning_radius, heading_delta);
            } else {
                self.wheel_angle += f32::max(-self.turning_radius, heading_delta);
            }

            if self.wheel_angle < -30. {
                self.wheel_angle = -30.;
            } else if self.wheel_angle > 30. {
                self.wheel_angle = 30.;
            }

            let radians = self.speed / (360./self.wheel_angle);

            self.heading += radians;
            self.heading %= 360.;

            let th = (self.heading / 180.0f32)*f32::consts::PI;
            self.location.y += th.cos() * self.pixels_per_tick();
            self.location.x += -th.sin() * self.pixels_per_tick();

            if self.speed < self.destination.speed {
                self.speed += self.acceleration.sqrt();
            } else {
                self.speed -= self.acceleration.sqrt();
            }

            if self.speed > 80. {
                self.speed = 80.;
            }

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
        let scale = 1.0;

        let mut shape = RectangleShape::new_init(&Vector2f::new(10.*scale, 20.*scale)).expect("Error, cannot draw car!!!");
        shape.set_position(&self.location);
        shape.set_origin(&Vector2f::new(5., 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = RectangleShape::new_init(&Vector2f::new(1.*scale, 50.*scale)).expect("Error, cannot draw line!!!");
        shape.set_position(&self.location);
        shape.set_origin(&Vector2f::new(0.5, 10.));
        shape.set_rotation(self.heading);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = RectangleShape::new_init(&Vector2f::new(1.*scale, 50.*scale)).expect("Error, cannot draw line!!!");
        shape.set_position(&self.location);
        shape.set_origin(&Vector2f::new(0.5, 10.));
        shape.set_rotation(self.heading + self.wheel_angle);
        shape.set_fill_color(&self.color);
        target.draw(&shape);

        let mut shape = CircleShape::new().expect("Error, cannot create ball.");
        shape.set_radius(5.*scale);
        shape.set_fill_color(&self.color);
        shape.set_position(&self.destination.location);
        shape.set_origin(&Vector2f::new(5., 5.));
        target.draw(&shape);
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
            let c = Color::new_rgb(rng.gen(), 0, 0);
            self.cars.push(Car::new(c, format!("{} {}", name, i), Box::new(DriveHomeDriver::new())));
        }
        self
    }

    pub fn tick(&mut self) {
        for _ in 0..50 {
            let shell_cars = self.cars.iter().map(|c| c.shell_copy()).collect();
            for i in 0..self.cars.len() {
                let _ = self.cars[(i+1)%(self.cars.len())].behind(30.);
                self.cars[i].tick(&shell_cars);
            }
        }
    }

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(self.cars.as_mut_slice());
    }
}

impl Drawable for Scenario {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        for car in self.cars.iter() {
            target.draw(car);
        }
    }
}
