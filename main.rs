use encryption::{xor_operation, column_mixing, create_s_box_table, row_shifting};

use decryption::{reverse_column_mixing, reverse_row_shifting, create_reverse_s_box_table};

use round_key_generator::generate_round_keys;

pub mod encryption;

pub mod decryption;

pub mod round_key_generator;

pub mod polynomials;

fn main() {

    let key: u128 = 0b00101111001100100010010000001100010110000001101011010100001000000100110011110001111111001010010011110101001001000101011101110011;

    let s_box = create_s_box_table();

    let mut round_keys: Vec<Vec<u8>> = generate_round_keys(key, s_box);

    let user_input = String::from("unfathomable huh").into_bytes();

    println!("og input: {:?}", user_input);

    let mut state = xor_operation(user_input, round_keys[0].clone());
    println!("{:?}", round_keys[0]);
    let s_box1 = create_s_box_table();
    
    for n in 1..10 {

    state = state.iter().map(|value| *s_box1.get(value).unwrap()).collect();
   
    state = row_shifting(state);

    state = column_mixing(state);

    state = xor_operation(state, round_keys[n].clone());

}
    println!("{:?}", state);
    state = state.iter().map(|value| *s_box1.get(value).unwrap()).collect();

    state = row_shifting(state);

    state = xor_operation(state, round_keys[10].clone());

    //decryption starts here.
    round_keys = round_keys.into_iter().rev().collect();

    state = xor_operation(state, round_keys[0].clone());

    state = reverse_row_shifting(state);

    let reverse_s_box = create_reverse_s_box_table();
    state = state.iter().map(|value| *reverse_s_box.get(value).unwrap()).collect();
    println!("{:?}", state);

    for n in 1..10 {

        state = xor_operation(state, round_keys[n].clone());

        state = reverse_column_mixing(state);

        state = reverse_row_shifting(state);

        state = state.iter().map(|value| *reverse_s_box.get(value).unwrap()).collect();
    
    }

    let decrypted = xor_operation(state, round_keys[10].clone());

    println!("{:?}", decrypted);

}
