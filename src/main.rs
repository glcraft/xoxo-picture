pub mod color_item;
pub mod trees;

use image;
use image::{GenericImage, GenericImageView};
use rand::{thread_rng, Rng};

use std::fs;
use std::collections::HashMap;

use color_item::ColorItem;
use trees::Octree;

fn get_img_color(path_pict: &str) -> [u8;3] {
    let mut total: [u64;3] = [0;3];
    let img = image::open(path_pict).unwrap();
    let (w,h)= (img.width(), img.height());
    
    for (_, _, pxl) in img.to_rgba().enumerate_pixels() {
        total[0] += pxl[0] as u64 * pxl[3] as u64 / 255;
        total[1] += pxl[1] as u64 * pxl[3] as u64 / 255;
        total[2] += pxl[2] as u64 * pxl[3] as u64 / 255;
    }
    [
        (total[0] / (w*h) as u64) as u8,
        (total[1] / (w*h) as u64) as u8,
        (total[2] / (w*h) as u64) as u8
    ]
}

fn create_base(path_assets: &str) -> Octree<ColorItem> {
    let mut bar = progress::Bar::new();
    bar.set_job_title("Reading pictures...");
    let t = fs::read_dir(path_assets).unwrap();
    let count = fs::read_dir(path_assets).unwrap().count();
    let mut ls_color_file: Vec<ColorItem> = Vec::new();
    for (i, file) in t.enumerate() {
        let filepath = file.unwrap().path();
        let filestem = filepath.file_stem();
        let file_str = filepath.to_str().unwrap();
        let file_stem_str = filestem.unwrap().to_str().unwrap();
        ls_color_file.push(ColorItem::new_one(get_img_color(file_str), String::from(file_stem_str)));
        bar.reach_percent((i * 100 / count) as i32);
    }
    bar.reach_percent(100);
    bar.jobs_done();
    println!("Generating trees...");
    Octree::generate(&mut ls_color_file[..])
}
fn main() {
    println!("xoxo picture!");
    const ASSETS_PATH: &str = "deps/twemoji/assets/72x72";
    let oct = create_base(ASSETS_PATH);
    let img = image::open("img_test.jpg").unwrap();
    let img = img.resize(img.width()/16, img.height()/16, image::imageops::FilterType::Gaussian);
    img.save("image_test_small.jpg");
    let mut img_new = image::RgbImage::new(img.width()*16, img.height()*16);
    let mut hs_emojies: HashMap<String, image::RgbImage> = HashMap::new();
    let str_null = String::new();
    let mut rng = thread_rng();
    for (x,y,pxl) in img.to_rgba().enumerate_pixels() {
        let idmoji = oct.get(&ColorItem::new_one([pxl[0],pxl[1],pxl[2]], str_null.clone()));
        if let Octree::Leaf(ls_moji) = idmoji {
            let emoji = &ls_moji[rng.gen_range(0,ls_moji.len())];
            if let Some(imoji) = hs_emojies.get(&emoji.files) {
                img_new.copy_from(imoji, x*16,y*16).expect("Image copy impossible");
            } else {
                let imoji = image::open(format!("{}/{}.png", ASSETS_PATH, emoji.files)).unwrap();
                let imoji = imoji.resize(16, 16, image::imageops::Gaussian).to_rgb();
                hs_emojies.insert(emoji.files.clone(), imoji);
            }
        }
    }
    img_new.save("img_test_result.png").expect("Unable to save picture");
}
