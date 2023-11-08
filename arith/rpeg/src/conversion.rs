use csc411_arith::index_of_chroma;
use csc411_arith::chroma_of_index;
use crate::format::RgbFloatValues;
use crate::codec::DCTValues;
use crate::format::PackedValues;
use std::borrow::Borrow;
use csc411_image::Rgb;

#[derive(Clone, Debug)]
pub struct Ypbpr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

/// Takes the index of chroma for the pb and pr values. Also converts b,c,d to 5 bits.
/// 
/// # Arguments:
/// * `pb_vector`: A signed integer value
/// * `new_width`: the width
/// * `_new_height`: the height 
/// * `row`: the row
/// * `col`: the col
pub fn dct(pb_vector: &Vec<Ypbpr>, new_width: u32, _new_height: u32, row: u32, col: u32) -> (f32, f32, f32, f32, usize, usize) {
    let top_left = pb_vector[(new_width * row + col) as usize].clone();
    let top_right = pb_vector[(new_width * row + (col + 1)) as usize].clone();
    let bot_left = pb_vector[(new_width * (row+1) + col) as usize].clone();
    let bot_right = pb_vector[(new_width * (row+1) + (col+1)) as usize].clone();

    let avg_pb = (top_left.pb + top_right.pb + bot_right.pb + bot_left.pb)/4.0;
    let avg_pr = (top_left.pr + top_right.pr + bot_right.pr + bot_left.pr)/4.0;

    let avg_pb = index_of_chroma(avg_pb as f32);
    let avg_pr = index_of_chroma(avg_pr as f32);

    let mut a = (bot_right.y + bot_left.y + top_right.y + top_left.y)/4.0;
    let mut b = (bot_right.y + bot_left.y - top_right.y - top_left.y)/4.0;
    let mut c = (bot_right.y - bot_left.y + top_right.y - top_left.y)/4.0;
    let mut d = (bot_right.y - bot_left.y - top_right.y + top_left.y)/4.0;

    a = (a* 511.0).round();
    b = (b.clamp(-0.3,0.3) * 50.0).round();
    c = (c.clamp(-0.3,0.3) * 50.0).round();
    d = (d.clamp(-0.3,0.3) * 50.0).round();


    return (a,b,c,d,avg_pb, avg_pr);
}

/// Converts rgb into a ypbpr values with given formula. Returns a vector including all the Ypbpr values.
/// 
/// # Arguments:
/// * `new_image`: &Vec<csc411_image::Rgb> holds the value of all the rgb pixels from the given file
/// * `new_image_deci`: Vec<RgbFloatValues> holds the decimal versions of the rgb pixels from new_image
/// * `new_width`: value to hold the width value
/// * `new_height`: value to hold the height value
pub fn rgbto_ypbpr(new_image: &Vec<csc411_image::Rgb>, new_image_deci: &Vec<RgbFloatValues>, new_width: u32, new_height: u32) -> Vec<Ypbpr>{
    let mut pb_vector: Vec<Ypbpr> = vec![Ypbpr{y: 0.0, pb:0.0, pr: 0.0}; new_width as usize * new_height as usize].clone();
    
    for pixel in 0..new_image.len(){
        let y = 0.299 * new_image_deci[pixel].red + 0.587 * new_image_deci[pixel].green + 0.114 * new_image_deci[pixel].blue;
        let pb = -0.168736 * new_image_deci[pixel].red + (-0.331264) * new_image_deci[pixel].green + 0.5 * new_image_deci[pixel].blue;
        let pr = 0.5 * new_image_deci[pixel].red + (-0.418688) * new_image_deci[pixel].green + (-0.081312) * new_image_deci[pixel].blue;
        pb_vector[pixel].y = y;
        pb_vector[pixel].pb = pb;
        pb_vector[pixel].pr = pr;

    }

    return pb_vector;
}


/// Calculates all the y values in a block through using DCT.
/// 
/// # Arguments:
/// * `dct_val_list`: list that holds the positions of 
/// * `_height`: value to hold the height value
/// * `_width`: value to hold the width value
/// * `unpack_word_list`: vector that contains the values needed to calculate y1,y2,y3,y4
pub fn dct_function(mut dct_val_list: Vec<DCTValues>, _height: u32, _width: u32, unpack_word_list: Vec<PackedValues>) -> Vec<DCTValues>{
    let mut counter = 0;
    for i in (0.._height).step_by(2){
        for j in (0.._width).step_by(2){
            let a_new = (unpack_word_list[counter].borrow().a as f32 / 511.0).clamp(0.0,1.0);
            let b_new = (unpack_word_list[counter].borrow().b as f32 / 50.0).clamp(-0.3,0.3);
            let c_new = (unpack_word_list[counter].borrow().c as f32 / 50.0).clamp(-0.3,0.3);
            let d_new = (unpack_word_list[counter].borrow().d as f32 / 50.0).clamp(-0.3,0.3);
            let pb = chroma_of_index(unpack_word_list[counter].borrow().avg_pb as usize);
            let pr = chroma_of_index(unpack_word_list[counter].borrow().avg_pr as usize);
            let y1 = a_new - b_new - c_new + d_new;
            let y2 = a_new - b_new + c_new - d_new;
            let y3 = a_new + b_new - c_new - d_new;
            let y4 = a_new + b_new + c_new + d_new;
            dct_val_list[(i * _width + j) as usize] = DCTValues{
                yval: y1,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_val_list[(i * _width + (j+1)) as usize] = DCTValues{
                yval: y2,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_val_list[((i+1) * _width + j) as usize] = DCTValues{
                yval: y3,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_val_list[((i+1) * _width + (j+1)) as usize] = DCTValues{
                yval: y4,
                avg_pb: pb,
                avg_pr: pr,
            };
            counter += 1;
        }
    }
    return dct_val_list;
}


/// Converts DCT values to rgb values as one of the last steps for decompression
/// 
/// # Arguments:
/// * `dct_val_list`: used to access the values to be converted to rgb
pub fn dct_to_rgb(dct_val_list: Vec<DCTValues>) -> Vec<Rgb>{
    //dct to rgb float
    let mut rgb_final = Vec::new();
    for value in dct_val_list{
        let rgb_val = Rgb{
            red: ((1.0 * value.yval + 0.0 * value.avg_pb + 1.402 * value.avg_pr) * 255.0) as u16,
            green: ((1.0 * value.yval - 0.344136 * value.avg_pb - 0.714136 * value.avg_pr) * 255.0) as u16,
            blue: ((1.0 * value.yval + 1.772 * value.avg_pb + 0.0 * value.avg_pr) * 255.0) as u16,
        };
        rgb_final.push(rgb_val);
    }
    return rgb_final;
}