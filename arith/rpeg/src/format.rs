use csc411_image::{RgbImage, Rgb};
use bitpack::bitpack::{gets, getu};

#[derive(Clone, Debug)]
pub struct RgbFloatValues {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct PackedValues{
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub avg_pb: u64,
    pub avg_pr: u64,
}


pub fn load_words(_raw_bytes: Vec<[u8; 4]>) -> Vec<PackedValues> {
    let mut unpack_word_list = Vec::new();
    for byte in _raw_bytes{
        // upacks chroma and avg pb and avg pr values
        let unpackedword = u32::from_be_bytes(byte);
        let a = getu(unpackedword as u64, 9, 23);
        let b = gets(unpackedword as u64, 5, 18);
        let c = gets(unpackedword as u64, 5, 13);
        let d = gets(unpackedword as u64, 5, 8);

        let avg_pb = getu(unpackedword as u64, 4, 4);
        let avg_pr = getu(unpackedword as u64, 4, 0);

        let packed = PackedValues{
            a: a,
            b: b,
            c: c,
            d: d,
            avg_pb,
            avg_pr,
        };
        // saves values above
        unpack_word_list.push(packed);
    }
    return unpack_word_list;
}

pub fn trim_img(read_in: &RgbImage, new_width: u32, new_height: u32) -> Vec<csc411_image::Rgb>{
    //vector to store values
    let mut new_image: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (new_height * new_width) as usize];

    //trimming last row if needed
    for i in 0..new_height{
        for j in 0..new_width{
            new_image[(new_width as usize * i as usize) + j as usize] = read_in.pixels[(read_in.width as usize * i as usize) + j as usize].clone();
        }
    }
    return new_image;
}

pub fn divide_denom(new_image: &Vec<csc411_image::Rgb>, read_in: &RgbImage, new_width: u32, new_height: u32) -> Vec<RgbFloatValues>{
    let mut new_image_deci: Vec<RgbFloatValues> = vec![RgbFloatValues{red: 0.0, green:0.0, blue: 0.0}; new_width as usize * new_height as usize].clone();

    //storing each pixel as a decimal value
    for pixel in 0..new_image.len(){
        new_image_deci[pixel].red = new_image[pixel].red as f32/(read_in.denominator as f32);
        new_image_deci[pixel].green = new_image[pixel].green as f32/read_in.denominator as f32;
        new_image_deci[pixel].blue = new_image[pixel].blue as f32/read_in.denominator as f32;
    }
    return new_image_deci;
}