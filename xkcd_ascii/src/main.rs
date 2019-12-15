extern crate tempfile;

use std::str::from_utf8;
use image::{GrayImage, DynamicImage};
use clap::{App};
use std::path::{Path, PathBuf};
use tempfile::{Builder};
use std::fs::File;
use std::{io, process};
use reqwest::Response;
use scraper::{Html, Selector};
use term_size;

fn main(){
    let matches = App::new("xkcd_ascii")
                    .version("0.0.1")
                    .author("flabbet")
                    .about("Turn an xkcd.com comic into ascii art!")
        .args_from_usage(
                        "[resize] -s, --resize [width] [height] 'Rescale image; does not preserve aspect ratio'
                               [comic_id] -i, --id [id] 'Sets the id of comic that will be converted'").get_matches();


    let comic_id = matches.value_of("comic_id").unwrap_or("");
    let (mut w, mut h) = term_size::dimensions().unwrap();

    if cfg!(target_os = "windows"){
        w+=1;
        h+=1;
    }
    
        let dims = match matches.values_of_lossy("resize") {
        Some(v) => v.iter().map(|s| s.parse::<u32>().unwrap()).collect(),
        None => vec![w as u32, h as u32]
    };

    let mut url:&str = &format!("https://xkcd.com/{}/", &comic_id);

    if comic_id.is_empty(){
        url = "https://c.xkcd.com/random/comic/";
    }

    let mut resp = generate_request(url).unwrap();
    let doc = Html::parse_document(&resp.text().unwrap());
    let comic_selector = Selector::parse("#comic > img").unwrap();

    let img = doc.select(&comic_selector).next().unwrap_or_else(||{
       println!("Could not find requested comic");
        process::exit(1)
    });
    let link = parse_to_valid_url(img.value().attr("src").unwrap());
    resp = generate_request(&link).unwrap();

    let tmp_dir = Builder::new().prefix("xkcd-comics").tempdir().expect("Failed to build temp dir");
    let fname = resp
             .url()
             .path_segments()
             .and_then(|segments| segments.last())
             .and_then(|name| if name.is_empty() { None } else { Some(name) })
             .unwrap_or("tmp.bin");

    println!("file to download: '{}'", fname);
    let fname = tmp_dir.path().join(fname);
    let file_name: &PathBuf = &fname;
    println!("will be located under: '{:?}'", fname);
    let mut file = File::create(&fname).expect("Failed to create file");
    io::copy(&mut resp, &mut file).expect("Failed to copy content");

    let mut img = open_image(file_name.to_str().unwrap());

    img = resize_image(img, &dims);
    let luma = img.to_luma();

    let ascii_art = to_ascii_art(img,&luma);
    print_ascii(chunk_string(&ascii_art, &luma));

}

fn parse_to_valid_url(url: &str) -> String{
    let prefix = "https://";
    let fixed_url = url
        .char_indices()
        .next()
        .and_then(|(i, _)| url.get(i+2..))
        .unwrap_or("");
    format!("{}{}",prefix, fixed_url)
}

fn generate_request<'a>(url: &str) -> reqwest::Result<Response> {
    let resp = reqwest::get(url)?;
    return Ok(resp);
}

fn open_image(path: &str) -> DynamicImage{
    let img = match image::open(&Path::new(path)) {
        Ok(p) => p,
        Err(e) => panic!("Not a valid image path or could no open image with error: {}", e.to_string()),
    };
    return img;
}

fn resize_image(img: DynamicImage, dims: &Vec<u32>) -> DynamicImage {
    return img.resize_exact(dims[0], dims[1], image::imageops::FilterType::Nearest);
}

fn to_ascii_art(_img: DynamicImage, buff: &GrayImage) -> String{
    return buff.pixels()
                    .map( |p| intensity_to_ascii(&p[0]))
                    .fold( String::new(), |s, p| s + p );
}

fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}

fn chunk_string<'a>(ascii_art: &'a String, img_buff: &'a GrayImage) -> Vec<&'a str>{
    let subs = ascii_art.as_bytes()
        .chunks(img_buff.width() as usize)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    return subs;
}

fn print_ascii(lines: Vec<&str>){
    for line in lines {
        print!("{}",line)
    }
}
