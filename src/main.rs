pub mod color_item;
pub mod trees;

use image;
use image::GenericImageView;

use std::fs;

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
    bar.set_job_title("Reading bar...");
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
    println!("Creating trees...");
    Octree::generate(&mut ls_color_file[..])
}
fn main() {
    println!("Hello, world!");
    let oct = create_base("deps/twemoji/assets/72x72");

}
