pub mod color_item;

use image;
use image::GenericImageView;

use std::fs;

fn create_base(path_assets: &str) {
    // let mut bar = progress::Bar::new();
    // bar.set_job_title("Creating base...");
    // bar.reach_percent(i * 10);
    println!("{:?}", fs::read_dir(path_assets).unwrap().count());
    for file in fs::read_dir(path_assets).unwrap() {
        print!("{}, ", file.unwrap().path().file_stem().unwrap().to_str().unwrap());
    }
}
fn main() {
    println!("Hello, world!");
    create_base("deps/twemoji/assets/72x72");

    // let img = image::open("img_test.jpg").unwrap();
    // let newimg = img.resize(img.width()/16, img.width()/16, image::imageops::FilterType::Gaussian);
    // newimg.save("img_test_result.png");
}
