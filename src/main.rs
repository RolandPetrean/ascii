use std::fs;

use image::GenericImageView;

const CHARS: &str = " `.,:;i1tfLCG08@";

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    println!("Usage: {} <image> <output>", args[0]);
    return;
  }
  let image_path = &args[1];
  let image = image::open(image_path).unwrap();
  let gray_image = image.grayscale();
  let (width, height) = gray_image.dimensions();
  
  let mut output = String::new();
  for i in 0..height {
    for j in 0..width {
      let pixel = gray_image.get_pixel(j, i);
      let index = (pixel[0] as f32/255.0*(CHARS.len() as f32-1.0)) as u8;
      output.push(CHARS.chars().nth(index as usize).unwrap());
    }
    output.push('\n');
  }
  
  fs::write(&args[2], output).expect("Unable to write to file");
}
