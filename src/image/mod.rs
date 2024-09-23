use color::Color;
use errors::ImageErrors;
use std::{fs::File, io::Write, usize};

pub mod color;
mod errors;

#[derive(Debug)]
pub struct Image {
    save_address: String,
    image_arr: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut img = Image {
            save_address: "".to_string(),
            image_arr: Vec::with_capacity(width * height),
            width,
            height,
        };
        img.image_arr = vec![Color::WHITE; width * height];
        img
    }

    pub fn set_save_address(&mut self, address: String) {
        self.save_address = address;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), ImageErrors> {
        if x > self.width {
            return Err(ImageErrors::OutOfBounds);
        }
        if y > self.height {
            return Err(ImageErrors::OutOfBounds);
        }
        println!("X: {}, Y: {}", x, y);
        self.image_arr[x + y * x] = color;

        Ok(())
    }

    pub fn save(&self) -> Result<(), ImageErrors> {
        let mut file = File::create(&self.save_address)?;
        let mut header = String::from("P6\n".to_string());
        header.push_str(&format!("{}\t{}\n", self.width, self.height));
        header.push_str("255\n");

        file.write_all(header.as_bytes())?;
        for color in &self.image_arr{
            file.write_all(&color.as_array())?;
        }


        Ok(())
    }
}
