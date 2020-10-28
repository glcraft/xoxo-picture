pub mod color_item;
pub mod trees;
pub mod args;

use clap::Clap;

use image;
use image::{GenericImage, GenericImageView};
use rand::{thread_rng, Rng};
use image::buffer::ConvertBuffer;

use std::fs;
use std::path::Path;
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
fn load_picture_base(result_base: &str) -> Vec<ColorItem> {
    let mut result = Vec::<ColorItem>::new();
    let lines = result_base.split("\n");
    'a: for line in lines {
        if line.len()==0 {
            continue;
        }
        let values: Vec<_> = line.split(";").collect();
        let str_color: Vec<_>  = values[0].split(",").collect();
        let mut color: [u8;3] = [0;3];
        for i in 0..3 {
            if let Ok(res) = str_color[i].trim().parse() {
                color[i] = res;
            } else {
                result.clear();
                break 'a;
            }
        }
        result.push(ColorItem::new_one(color, String::from(values[1])));
    }
    result
}

fn save_picture_base(ls_colors: &Vec<ColorItem>) -> String {
    let mut result = String::new();
    for color in ls_colors {
        let formatted = format!("{},{},{};{}\n", color.color[0], color.color[1], color.color[2], color.files);
        result.push_str(&formatted);
    }
    result
}

fn load_emoji(path: &String, size: (u32, u32)) -> image::RgbImage {
    let imoji = image::open(&path).expect(&format!("le chemin suivant est introuvable: {}", path));
    let mut imoji = imoji.resize(size.0, size.1, image::imageops::Gaussian).to_rgba();
    for (_,_,pxl) in imoji.enumerate_pixels_mut() {
        pxl[0] = (pxl[0] as u16 * pxl[3] as u16 / 255) as u8;
        pxl[1] = (pxl[1] as u16 * pxl[3] as u16 / 255) as u8;
        pxl[2] = (pxl[2] as u16 * pxl[3] as u16 / 255) as u8;
    }
    imoji.convert()
}

fn create_base(path_assets: &str, path_dir_exe: std::path::PathBuf) -> Octree<ColorItem> {
    let mut ls_color_file: Vec<ColorItem> = Vec::new();
    let path_base = format!("{}/base.txt",path_dir_exe.to_str().unwrap());
    if let Ok(result_base) = fs::read_to_string(&path_base) {
        ls_color_file = load_picture_base(&result_base);
    } 
    if ls_color_file.len()==0 {
        let mut bar = progress::Bar::new();
        bar.set_job_title("Reading pictures...");
        let t = match fs::read_dir(path_assets) {
            Err(_) => panic!("Assets path not found '{}'", path_assets),
            Ok(dir_entry) => dir_entry
        };
        let count = fs::read_dir(path_assets).unwrap().count();
        
        for (i, file) in t.enumerate() {
            let filepath = file.unwrap().path();
            let filename = filepath.file_name();
            let file_str = filepath.to_str().unwrap();
            let file_name_str = filename.unwrap().to_str().unwrap();
            ls_color_file.push(ColorItem::new_one(get_img_color(file_str), String::from(file_name_str)));
            bar.reach_percent((i * 100 / count) as i32);
        }
        bar.reach_percent(100);
        bar.jobs_done();
        println!("Saving pictures base into {}...", path_base);
        if let Err(err) = fs::write(path_base, save_picture_base(&ls_color_file)) {
            println!("Error writing base.txt (reason: {}), continue...", err);
        }
    }
    println!("Generating trees...");
    Octree::generate(&mut ls_color_file[..])
}
fn main() {
    let mut args: args::Opts = args::Opts::parse();
    let path_dir_exe = std::env::current_exe().expect("Unable to get the executable path").parent().expect("Unable to get the parent of the executable path").to_path_buf();
    if let None = args.assets_path {
        let path_assets = format!("{}/{}", path_dir_exe.to_str().unwrap(), if cfg!(debug_assertion) {"deps/twemoji/assets/72x72"} else {"deps/twemoji"});
        args.assets_path = Some(path_assets);
    }
    let assets_path = args.assets_path.unwrap();
    let size_upscale = args.size*args.upscale;
    println!("xoxo picture!");
    let oct = create_base(&assets_path, path_dir_exe);
    let img = image::open(&args.filename).unwrap();
    let img = img.resize(img.width()/args.size, img.height()/args.size, image::imageops::FilterType::Gaussian);
    // img.save("image_test_small.jpg");
    let mut img_new = image::RgbImage::new(img.width()*size_upscale, img.height()*size_upscale);
    let mut hs_emojies: HashMap<String, image::RgbImage> = HashMap::new();
    let str_null = String::new();
    let mut rng = thread_rng();
    for (x,y,pxl) in img.to_rgba().enumerate_pixels() {
        let idmoji = oct.get(&ColorItem::new_one([pxl[2],pxl[1],pxl[0]], str_null.clone()));
        if let Octree::Leaf(ls_moji) = idmoji {
            let emoji = &ls_moji[rng.gen_range(0,ls_moji.len())];
            if let Some(imoji) = hs_emojies.get(&emoji.files) {
                img_new.copy_from(imoji, x*size_upscale,y*size_upscale).expect("Unable to copy emoji into the new picture");
            } else {
                let path = format!("{}/{}", assets_path, emoji.files);
                let imoji = load_emoji(&path, (size_upscale,size_upscale));
                img_new.copy_from(&imoji, x*size_upscale,y*size_upscale).expect("Unable to copy emoji into the new picture");
                hs_emojies.insert(emoji.files.clone(), imoji);
            }
        }
    }
    let new_path = Path::new(&args.filename);
    let parent_path = new_path.parent().unwrap();
    let new_path = format!("{}{}_result.{}",{
        let parent_str = parent_path.to_str().unwrap();
        if parent_str.len()>0 {
            format!("{}/", parent_str)
        } else {
            String::from("")
        }
    },new_path.file_stem().unwrap().to_str().unwrap(),new_path.extension().unwrap().to_str().unwrap());
    img_new.save(new_path).expect(&format!("Unable to save picture to {}", new_path));//format!("Unable to save picture to {}", new_path)
}
