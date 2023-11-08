use csc411_image;
use csc411_rpegio;
use csc411_image::Write;
use csc411_image::{Read, RgbImage};
use bitpack::bitpack::{newu, news};
use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
use crate::format::{trim, divide_denom, load_words};
use crate::conversion::{rgbto_ypbpr, dct, dct_function, dct_to_rgb};

// created structs to easier manipulate data
#[derive(Clone, Debug)]
pub struct Ypbpr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

#[derive(Clone, Debug)]
pub struct RgbFloatValues {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct DCTValues{
    pub yval: f32,
    pub avg_pb: f32,
    pub avg_pr: f32,
}

// function for compression
pub fn compress(filename: &str){
    let read_in = RgbImage::read(Some(filename)).unwrap();

    let mut new_width = read_in.width;
    let mut new_height = read_in.height;

    //trimming
    if read_in.width % 2 != 0{
        new_width -= 1;
    }
    if read_in.height % 2 != 0{
        new_height -=1;
    }
    
    let new_image = trim(&read_in, new_width, new_height);
    
    //new vector to store decimal 
    let new_image_deci = divide_denom(&new_image, &read_in, new_width, new_height);
    
    //vector for storing Ypbpr values from the original RGB values
    let pb_vector = rgbto_ypbpr(&new_image, &new_image_deci, new_width, new_height);

    //converting from rgb to component video 
    let mut word_vec = Vec::new();
    for row in (0..new_height).step_by(2){
        for col in (0..new_width).step_by(2){
            let (a,b,c,d,avg_pb,avg_pr) = dct(&pb_vector, new_width, new_height, row, col);
            let mut word = 0_u64;
            word = newu(word, 9, 23, a as u64).unwrap();
            word = news(word, 5, 18, b as i64).unwrap();
            word = news(word, 5, 13, c as i64).unwrap();
            word = news(word, 5, 8, d as i64).unwrap();
            word = newu(word, 4, 4, avg_pb as u64).unwrap();
            word = newu(word, 4, 0, avg_pr as u64).unwrap();
            word_vec.push((word as u32).to_be_bytes());
        }
    }
    output_rpeg_data(&word_vec, new_width, new_height);
}

// Decompress----------------------------------------------------------------------------------------
pub fn decompress(filename: &str) {
    let (_raw_bytes, _width, _height) = read_in_rpeg_data(Some(filename)).unwrap();
    
    // reads in compressed image data
    let unpack_word_list = load_words(_raw_bytes);

    // vector for keeping track of values when converted through DCT
    let mut dct_val_list: Vec<DCTValues> = vec![DCTValues{yval: 0.0, avg_pb: 0.0, avg_pr: 0.0}; _height as usize* _width as usize];

    //converts a,b,c,d values to ypbpr and stores them
    dct_val_list = dct_function(dct_val_list, _height, _width, unpack_word_list);
    
    //converts ypbpr to rgb 
    let rgb_final = dct_to_rgb(dct_val_list);

    //writing final RGB image out
    let final_image = RgbImage{
        width: _width as u32,
        height: _height as u32,
        denominator: 255,
        pixels: rgb_final,
    };
                
    final_image.write(None).unwrap();
}