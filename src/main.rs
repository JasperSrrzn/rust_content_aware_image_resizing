#[macro_use]
extern crate log;
extern crate pretty_env_logger;

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

    #[structopt(short = "n", default_value = "1")]
    times: u32,
}

fn main() {
    pretty_env_logger::init();
    let options = Options::from_args();
    let mut image = ImageWrapper::from_file(&options.inputpath);
    info!("The original image has a size: {:?}x{:?}.", image.get_image().width(), image.get_image().height());


    for n in 0..options.times {
        info!("Iteration {}", n);
        image = crop(&image);
        info!("The image now has a size: {}x{}.", image.get_image().width(), image.get_image().height());
        info!("==========================");
    }

    info!("Finished with an image size of {}x{}", image.get_image().width(), image.get_image().height());
    let image_result = image.get_image().save(&options.outputpath);
    match image_result {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}






