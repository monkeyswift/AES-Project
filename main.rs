use encryption::{apply_xor_operation, arrange_into_grid, column_mixing, create_s_box_table, row_shifting};

use decryption::{reverse_column_mixing, reverse_row_shifting, create_reverse_s_box_table};

use round_key_generator::{rot_word_transformation, sub_word, round_constants_generator};

pub mod encryption;

pub mod decryption;

pub mod round_key_generator;

pub mod polynomials;

//I will try to implement some type of AES encryption algorithm here
// My key will be 128 bits and plaintext should be about 16 bytes.
// the 16 bytes will be arranged into a 4x4 grid. The process below will occur 10 times in accordance with the AES standard.
// Xor operation -> substitute bytes -> shift rows -> mix columns.
// the grid looks like this:
// [x_0, x_4, x_8,  x_12]
// [x_1, x_5, x_9,  x_13]
// [x_2, x_6, x_10, x_14]
// [x_3, x_7, x_11, x_15]

// This is how I shall model polynomials:
// A struct will hold each unit of the polynomial
//
//I am going to iterate 

fn main() {
    //let limbo_string = String::from(lib::accept_user_input());

    let key = 237282396920938463463374607431768715456 as u128;

    let user_input = String::from("unfathomable huh");

    // since this is just a demonstrative implementation I am going to refrain from constructing an initial vector generator. Maybe I'll change it in the future if I decide to
    //add more features.
    let mut dynamic_vector: u128 = 0b00101111001100100010010000001100010110000001101011010100001000000100110011110001111111001010010011110101001001000101011101110011;
    let mut state = dynamic_vector.to_ne_bytes().to_vec();
    //xor goes here

    let s_box = create_s_box_table();
    state = state.iter().map(|value| *s_box.get(value).unwrap()).collect();
    state = row_shifting(state);
    state = column_mixing(state);

    let mut key_vector: Vec<Vec<u8>> = key.to_ne_bytes().chunks(4).map(|x|x.to_vec()).collect();

    for _ in 0..9 {
        
        let mut key_vector_iter = key_vector.clone().into_iter();

        let key_vec1 = key_vector_iter.next().unwrap();
        let key_vec2 = key_vector_iter.next().unwrap();
        let key_vec3 = key_vector_iter.next().unwrap();
        let key_vec4 = key_vector_iter.next().unwrap();

        let mut travler = rot_word_transformation(key_vec4.clone());

        travler = travler.iter().map(|value| *s_box.get(value).unwrap()).collect();

        travler = round_constants_generator(travler);

    }

}

fn process_into_blocks(mut user_input: String) -> () {
    let mut processed_input: Vec<String> = Vec::new();
    while user_input.len() > 16 {
        processed_input.push(user_input.drain(0..16).collect());
    }

    if user_input.len() >= 1 {
    let mut remains = user_input.clone();
    for _ in 0..16 - user_input.len() {
        remains.push('a')
    }
    processed_input.push(remains);
}
}

#[cfg(test)]
mod tests {
    //use super::*;
}
