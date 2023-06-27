use encryption::{xor_operation, column_mixing, create_s_box_table, row_shifting};

use decryption::{reverse_column_mixing, reverse_row_shifting, create_reverse_s_box_table};

use round_key_generator::generate_round_keys;

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

    let key: u128 = 0b00101111001100100010010000001100010110000001101011010100001000000100110011110001111111001010010011110101001001000101011101110011;

    let s_box = create_s_box_table(); //probably going to change this in the future. Created an exta hashmap to pass into the key generation for orginazation's sake.

    let round_keys = generate_round_keys(key, s_box);

    let mut user_input = String::from("unfathomable huh").into_bytes();

    println!("{:?}", user_input);

    let mut state = xor_operation(user_input, key.to_ne_bytes().to_vec());

    let s_box1 = create_s_box_table();
    state = state.iter().map(|value| *s_box1.get(value).unwrap()).collect();
    state = row_shifting(state);
    state = column_mixing(state);
    println!("{:?}", state);
    state = reverse_column_mixing(state);
    state = reverse_row_shifting(state);
    let s_box2 = create_reverse_s_box_table();
    state = state.iter().map(|value| *s_box2.get(value).unwrap()).collect();
    state = xor_operation(state, key.to_ne_bytes().to_vec());
    println!("{:?}", state);

}
