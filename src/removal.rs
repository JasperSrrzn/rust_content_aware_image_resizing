use std::time::Instant;
use image::{DynamicImage, GenericImageView, Pixel, RgbImage};
use image_energy::energy_of_image::image_wrapper::ImageWrapper;
use crate::get_minimum_seam;

pub fn crop(image: &ImageWrapper) -> ImageWrapper {
    let mut start = Instant::now();
    let energy = image.get_energy();
    let mut duration = start.elapsed();
    debug!("Energy calculation took {:?}", duration);
    start = Instant::now();
    let result = get_minimum_seam(&image, &energy);
    duration = start.elapsed();
    debug!("Finding seam took {:?}", duration);
    start = Instant::now();
    let img = remove_seam_from_image(result, image.get_image());
    duration = start.elapsed();
    debug!("Stripping took {:?}", duration);
    ImageWrapper::new(img)
}


fn remove_seam_from_image(seam: Vec<(u32, u32)>, image: &DynamicImage) -> DynamicImage {
    let mut img = RgbImage::new(image.width() - 1, image.height());
    image.pixels().for_each(|(x, y, c)| {
        let pixel_to_remove: &(u32, u32) = seam.get(y as usize).unwrap();
        if x < pixel_to_remove.0 {
            img.put_pixel(x, y, c.to_rgb());
        }

        if x > pixel_to_remove.0 {
            img.put_pixel(x - 1, y, c.to_rgb());
        }
    });
    DynamicImage::from(img)
}