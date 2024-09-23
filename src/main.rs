use image::Image;
use image::color::Color;

mod image;
const WIDTH: usize = 255;
const HEIGHT: usize = 255;

fn main() {
    let mut image = Image::new(WIDTH, HEIGHT);
    image.set_save_address("./noise.ppm".to_string());

    for y in 0..WIDTH{
        for x in 0..HEIGHT{
            let _ = image.set_pixel(x, y, Color::new(
                (x % 255) as u8,
                (y % 255) as u8,
                128
            ));
        }
    }

    let result = image.save();

    match result {
        Err(error) => {
            println!("{}", error);
        },
        Ok(()) => {
            println!("File was created successfully!");
        }
    }


}
