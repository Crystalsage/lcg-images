use std::fs::File;
use std::io::prelude::*;

mod lcg;

const WIDTH: i64 = 1920;
const HEIGHT: i64 = 1080;

static FILE_PATH: &str = "output.ppm";

type Color32 = i64;
type Image = Vec<Vec<Color32>>;

const BRIGHT_RED :Color32       = 0xFF3449FB;
const BRIGHT_GREEN :Color32     = 0xFF26BBB8;
const BRIGHT_YELLOW:Color32     = 0xFF2FBDFA;
const BRIGHT_BLUE :Color32      = 0xFF98A583;
const BRIGHT_PURPLE:Color32     = 0xFF9B86D3;
const BRIGHT_AQUA :Color32      = 0xFF7CC08E;
const BRIGHT_ORANGE :Color32    = 0xFF1980FE;
const COLOR_BACKGROUND: Color32 = 0xFF181818;

fn make_image() -> Image {
    let mut image: Image = vec![vec![0_i64; WIDTH as usize]; HEIGHT as usize];
    let color_palette = vec![
        BRIGHT_RED,
        BRIGHT_GREEN,
        BRIGHT_YELLOW,
        BRIGHT_BLUE,
        BRIGHT_PURPLE,
        BRIGHT_AQUA,
        BRIGHT_ORANGE,
        COLOR_BACKGROUND,
    ];

    let random_nums = lcg::get_random_numbers(WIDTH * HEIGHT);

    let mut j = 0;
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let color = color_palette[(random_nums[j as usize] % color_palette.len() as u64) as usize];
            image[y][x] = color;
            j += 1;
        }
    }

    return image;
}

fn save_image_as_ppm(image: Image) -> std::io::Result<()> {
   let mut file = File::create(FILE_PATH)?;
   file.write_all(format!("P6\n{} {} 255\n", WIDTH, HEIGHT).as_bytes())?;

   let mut all_bytes: Vec<u8> = Vec::new();

   for y in 0..HEIGHT as usize {
       for x in 0..WIDTH as usize {
           let pixel: i64 = image[y][x];

           // Extract red component
           all_bytes.push(((pixel&0x0000FF) >> 8*0) as u8);

           // Extract green component
           all_bytes.push(((pixel&0x00FF00) >> 8*1) as u8);

           // Extract blue component
           all_bytes.push(((pixel&0xFF0000) >> 8*2) as u8);
       }
   }

   file.write_all(&all_bytes).unwrap();

   Ok(())
}

fn main() {
    // Recipe
    let image: Image = make_image();
    save_image_as_ppm(image).unwrap();
}
