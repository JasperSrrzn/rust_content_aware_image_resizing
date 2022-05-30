use std::cmp::min;
use std::collections::HashMap;
use image_energy::energy_of_image::image_wrapper::ImageWrapper;
use crate::SeamPixel;


pub fn get_minimum_seam(image: &ImageWrapper, energy: &HashMap<(u32, u32), u32>) -> Vec<(u32, u32)> {
    let seams = calculate_seams(image, energy);
    let mut minimum_position: (u32, u32) = (0, 0);
    let mut minimum_seam = u32::MAX;
    for x in 0..image.get_image().width() - 1 {
        let seam = seams.get(&(x, image.get_image().height() - 1)).unwrap();
        if seam.seam < minimum_seam {
            minimum_position = (x, image.get_image().height() - 1);
            minimum_seam = seam.seam;
        }
    }
    let mut result = Vec::new();
    let mut next = Some(minimum_position);
    while next.is_some() {
        result.push(next.unwrap());
        next = seams.get(&next.unwrap()).unwrap().pointer;
    }
    result.reverse();
    return result;
}

fn calculate_seams(image: &ImageWrapper, energy: &HashMap<(u32, u32), u32>) -> HashMap<(u32, u32), SeamPixel> {
    let mut seams = HashMap::new();
    for y in 0..image.get_image().height() {
        for x in 0..image.get_image().width() {
            if y == 0 {
                let seam = get_energy(x, y, energy);
                seams.insert((x, y), SeamPixel::new(seam, None));
            } else {
                let mut seam = get_energy(x, y, energy);
                let up = seams.get(&(x, y - 1)).unwrap().seam;
                if x == 0 {
                    let right = seams.get(&(x + 1, y - 1)).unwrap().seam;
                    let min = min(right, up);
                    seam += min;
                    if right == min {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x + 1, y - 1))));
                    } else {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x, y - 1))));
                    }
                } else if x == image.get_image().width() - 1 {
                    let left = seams.get(&(x - 1, y - 1)).unwrap().seam;
                    let min = min(left, up);
                    seam += min;
                    if left == min {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x - 1, y - 1))));
                    } else {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x, y - 1))));
                    }
                } else {
                    let left = seams.get(&(x - 1, y - 1)).unwrap().seam;
                    let right = seams.get(&(x + 1, y - 1)).unwrap().seam;
                    let min = min(left, min(up, right));
                    seam += min;
                    if min == left {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x - 1, y - 1))));
                    } else if min == right {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x + 1, y - 1))));
                    } else {
                        seams.insert((x, y), SeamPixel::new(seam, Some((x, y - 1))));
                    }
                }
            }
        }
    }
    seams
}

fn get_energy(x: u32, y: u32, energy: &HashMap<(u32, u32), u32>) -> u32 {
    return *energy.get(&(x, y)).unwrap();
}


