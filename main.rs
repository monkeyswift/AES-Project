use encryption::{
    apply_xor_operation, arrange_into_grid, column_mixing, create_s_box_table, row_shifting,
};
use round_key_generator::rot_word_transformation;

pub mod encryption;

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
//

fn main() {
    //let limbo_string = String::from(lib::accept_user_input());

    let key = String::from(format!(
        "{:0128b}",
        237282396920938463463374607431768715456 as u128
    ));

    round_key_generator(key);

    let user_input = String::from("unfathomable huh");

    let mut state = apply_xor_operation(user_input);

    let s_box = create_s_box_table();

    state = state
        .iter()
        .map(|value| *s_box.get(value).unwrap())
        .collect(); //using the s-box

    state = row_shifting(state);
    println!("{:?}", state);
    column_mixing(state);
}

fn round_key_generator(key: String) {
    let grided_key = arrange_into_grid(key);

    rot_word_transformation(grided_key);
}

#[cfg(test)]
mod tests {
    //use super::*;
}
