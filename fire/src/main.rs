extern crate piston_window;

use piston_window::*;
use rand::{Rng, thread_rng};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;
const COLOR_RANGE: u8 = 64;
const ALPHA_CHANNEL: f32 = 1.0;
const LOW_COLOR_COMPONENT: f32 = 0.0;
const HIGH_COLOR_COMPONENT: f32 = 1.0;

fn main() {
    // Declare internal buffer for calculations.
    let mut buffer = vec![vec![0; (WIDTH + 2) as usize]; (HEIGHT / 2) as usize];

    // Create 2D graphics window.
    let mut window: PistonWindow =
        WindowSettings::new("Fire", [WIDTH, HEIGHT])
            .exit_on_esc(true).build().unwrap();

    // Make initialization. Fill the background with black color.
    if let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _device| {
            clear([0.0; 4], g);
        });
    }

    // Main loop.
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            do_flame(c, g, &mut buffer);
        });
    }
}

fn temperature_to_color(temperature: u8) -> types::Color {
    if temperature < COLOR_RANGE + COLOR_RANGE / 2 {
        // The lowest temperatures are shown as black color.
        return [LOW_COLOR_COMPONENT, LOW_COLOR_COMPONENT, LOW_COLOR_COMPONENT, ALPHA_CHANNEL];
    } else if temperature < COLOR_RANGE * 2 + COLOR_RANGE / 2 {
        // The next range of temperatures is in the range from black to red color.
        return [(temperature - (COLOR_RANGE + COLOR_RANGE / 2)) as f32 / (COLOR_RANGE - 1) as f32, LOW_COLOR_COMPONENT, LOW_COLOR_COMPONENT, ALPHA_CHANNEL];
    } else if temperature < COLOR_RANGE * 3 + COLOR_RANGE / 2 {
        // The next range of temperatures is in the range from red to yellow color.
        return [HIGH_COLOR_COMPONENT, (temperature - (COLOR_RANGE * 2 + COLOR_RANGE / 2)) as f32 / (COLOR_RANGE - 1) as f32, LOW_COLOR_COMPONENT, ALPHA_CHANNEL];
    }

    // The last range of temperatures is in the range from yellow to white color.
    [HIGH_COLOR_COMPONENT, HIGH_COLOR_COMPONENT, (temperature - (COLOR_RANGE * 3 + COLOR_RANGE / 2)) as f32 / (COLOR_RANGE - 1) as f32, ALPHA_CHANNEL]
}

fn fill_temperatures(buffer: &mut Vec<Vec<u8>>) {
    let mut rng = thread_rng();
    for x in (0..WIDTH + 2).rev() {
        buffer[(HEIGHT / 2 - 1) as usize][x as usize] = rng.gen_range(0, 255) as u8;
    }
}

fn do_one_step_calculation(buffer: &mut Vec<Vec<u8>>) {
    for y in (1..(HEIGHT / 2 - 1) as usize).rev() {
        for x in (1..=(WIDTH) as usize).rev() {
            let origin = buffer[y][x] as u16;
            let bottom = buffer[y + 1][x] as u16;
            let bottom_left = buffer[y + 1][x - 1] as u16;
            let bottom_rigth = buffer[y + 1][x + 1] as u16;
            buffer[y][x] = ((origin + bottom + bottom_left + bottom_rigth) / 4) as u8;
        }
    }
}

fn draw(c: Context, g: &mut G2d, buffer: &mut Vec<Vec<u8>>) {
    for y in 0..HEIGHT / 2 - 1 {
        for x in 0..WIDTH {
            let color = temperature_to_color(buffer[y as usize][(x + 1) as usize]);
            rectangle(color,
                      [x as f64, ((y + 1) * 2) as f64, 1.0, 2.0],
                      c.transform, g);
        }
    }
}

fn do_flame(c: Context, g: &mut G2d, buffer: &mut Vec<Vec<u8>>) {
    fill_temperatures(buffer);
    do_one_step_calculation(buffer);
    draw(c, g, buffer);
}
