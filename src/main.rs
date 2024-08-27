extern crate piston_window;

use piston_window::*;

const G: f64 = 6.67430e-11;
const DT: f64 = 60. * 60. * 24.;

struct Body {
    x: f64,
    y: f64,
    mass: f64,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
}

impl Body {
    fn new(x: f64, y: f64, mass: f64, vx: f64, vy: f64) -> Body {
        Body {
            x,
            y,
            mass,
            vx,
            vy,
            ax: 0.,
            ay: 0.,
        }
    }

    fn update_velocity(&mut self) {
        self.vx += self.ax * DT;
        self.vy += self.ay * DT;
    }

    fn update_position(&mut self) {
        self.x += self.vx * DT + 0.5 * self.ax * DT * DT;
        self.y += self.vy * DT + 0.5 * self.ay * DT * DT;
    }
}

fn update_acceleration_all_bodies(bodies: &mut [Body]) {
    let n = bodies.len();

    for i in 0..n {
        bodies[i].ax = 0.0;
        bodies[i].ay = 0.0;
        for j in 0..n {
            if i == j {
                continue;
            }

            let dx = bodies[j].x - bodies[i].x;
            let dy = bodies[j].y - bodies[i].y;
            let dist_sq = dx * dx + dy * dy;
            let force = G * bodies[j].mass / dist_sq;
            let distance = dist_sq.sqrt();
            bodies[i].ax += force * dx / distance;
            bodies[i].ay += force * dy / distance;
        }
    }
}

fn simulate(bodies: &mut [Body]) {
    update_acceleration_all_bodies(bodies);

    for body in bodies {
        body.update_position();
        body.update_velocity();
    }
}

fn main() {
    let mut bodies = [
        Body::new(0., 0., 1.989e30, 0., 0.),           // Sun
        Body::new(57.9e9, 0., 3.285e23, 0., 47.87e3),  // Mercury
        Body::new(108.2e9, 0., 4.867e24, 0., 35.02e3), // Venus
        Body::new(149.6e9, 0., 5.972e24, 0., 29.78e3), // Earth
        Body::new(227.9e9, 0., 6.39e23, 0., 24.07e3),  // Mars
        Body::new(778.5e9, 0., 1.898e27, 0., 13.07e3), // Jupiter
    ];

    let window_width = 800;
    let window_height = 600;

    let mut window: PistonWindow =
        WindowSettings::new("N-body Simulation", [window_width, window_height])
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    window.set_max_fps(60);

    while let Some(event) = window.next() {
        // Update body positions
        simulate(&mut bodies);

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let transform = c
                .transform
                .trans((window_width / 2) as f64, (window_height / 2) as f64);

            for body in &bodies {
                ellipse(
                    [1.0, 1.0, 1.0, 1.0],                   // white
                    [body.x / 3e9, body.y / 3e9, 5.0, 5.0], // x, y, width, height
                    transform,
                    g,
                );
            }
        });
    }
}
