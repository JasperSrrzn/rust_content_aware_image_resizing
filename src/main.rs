use std::cmp::min;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgb, Rgba, RgbImage};
use image_energy::energy_of_image::image_wrapper::ImageWrapper;
use structopt::StructOpt;
use crate::removal::crop;
use crate::seam::get_minimum_seam;

use crate::seam_pixel::SeamPixel;

mod seam_pixel;
mod seam;
mod removal;


#[derive(StructOpt)]
struct Options {
    #[structopt(short = "i")]
    inputpath: String,

    #[structopt(short = "o")]
    outputpath: String,
}

fn main() {
    let options = Options::from_args();
    let mut image = ImageWrapper::from_file(&options.inputpath);

    println!("{}x{}", image.get_image().width(), image.get_image().height());

    for n in 0..500 {
        println!("Iteration {}", n);
        image = crop(&image);
        let image_result = image.get_image().save(format!("./images/video_france/output_{}.jpg", n));
        match image_result {
            Ok(_) => {}
            Err(e) => panic!("{}", e)
        }
        println!("==========================");
    }


    println!("{}x{}", image.get_image().width(), image.get_image().height());
    let image_result = image.get_image().save(&options.outputpath);
    match image_result {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}






