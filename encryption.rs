use std::collections::HashMap;
use std::io::{self, BufRead};
use crate::polynomials::{polynomial_conversion, Polynomial};



pub fn accept_user_input() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().fuse();
    let mut ext_input = String::new();

    loop {
        let input = match lines.next() {
            Some(Ok(line)) => line,
            _ => panic!("Couldn't read input."),
        };

        if input.trim().is_empty() {
            ext_input = input;
            break;
        }
    }
    ext_input
}

//the key is 237282396920938463463374607431768715456.
//let key: String = format!("{:0128b}", 237282396920938463463374607431768715456 as u128);
//convert limbo into it's bit representation by iterating over every character, then formatting that character into it's bit representation ("{:08b}"). Use .for_each() to
//push_str(the formatted character) into limbo_cipher.
//then hand implement the Xor operator using expanded_limbo_cipher.chars.zip(key.chars()).for_each(for now just do .push_str() but toy with spewing out a complete grid.)
// <- if that doesn't work use the arrange_into_grid function you made.

pub fn apply_xor_operation(user_input: String) -> Vec<u8> {
    let custom_key: u128 = 237282396920938463463374607431768715456;

    let user_string_bits = user_input.as_bytes(); //string as bytes
    let custom_key_bits = custom_key.to_ne_bytes(); //integer as bytes

    let state = user_string_bits
        .iter()
        .zip(custom_key_bits.iter())
        .map(|(user_string_bytes, custom_key_bytes)| user_string_bytes ^ custom_key_bytes)
        .collect::<Vec<_>>();

    state
}

pub fn arrange_into_grid(input: String) -> Vec<String> {
    let mut grid: Vec<String> = Vec::new();

    let mut iterator_of_xor_d_s = input.chars();

    let mut inner_vector = Vec::new();

    for n in 1..=128 {
        inner_vector.push(iterator_of_xor_d_s.next().unwrap());

        if n % 8 == 0 {
            let stringified_vec = inner_vector.iter().collect::<String>().clone();
            grid.push(stringified_vec);
            inner_vector.clear();
        }
    }
    grid
}

// gonna create the s-box hashmap right here, need to create it outside of the byte_sub function so that it only needs to be generated once

//the following code isn't the most memory efficient but I think its function is clearer. I could have just done for n in 256 instead of making all the x and y axis crap
//but it makes it more clear how the sbox is implemented here.

pub fn create_s_box_table() -> HashMap<u8, u8> {
    let mut s_box: HashMap<u8, u8> = HashMap::new();

    let y_axis: Vec<&str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];

    let x_axis: Vec<&str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];

    let input_byte_vector: Vec<u8> = y_axis
        .iter()
        .flat_map(|y| {
            x_axis
                .iter()
                .map(move |x| u8::from_str_radix(format!("{}{}", y, x).as_str(), 16).unwrap())
        })
        .collect();

    let output_byte_vector: Vec<u8> = vec![
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];

    input_byte_vector
        .into_iter()
        .zip(output_byte_vector.into_iter())
        .for_each(|(a, b)| {
            s_box.insert(a, b);
        });

    s_box
}

pub fn row_shifting(mut state: Vec<u8>) -> Vec<u8> {
    let mut counter: u8 = 0;

    state = state
        .chunks(4)
        .flat_map(|value: &[u8]| -> Vec<u8> {
            counter += 1;
            match counter {
                1 => {
                    let val_as_vec = value.to_vec();
                    val_as_vec
                }
                2 => {
                    let mut val_as_vec = value.to_vec();
                    let first_val = val_as_vec.remove(0);
                    val_as_vec.push(first_val);
                    val_as_vec
                }
                3 => {
                    let mut val_as_vec = value.to_vec();
                    let first_two: Vec<u8> = val_as_vec.drain(..2).collect();
                    val_as_vec.extend(first_two);
                    val_as_vec
                }
                4 => {
                    counter = 0;
                    let mut val_as_vec = value.to_vec();
                    let last_val = val_as_vec.remove(3);
                    val_as_vec.insert(0, last_val);
                    val_as_vec
                }
                _ => value.to_vec(),
            }
        })
        .collect::<Vec<_>>();

    state
}

pub fn column_mixing(state: Vec<u8>) -> Vec<u8> {
    let mut state_1 = state.chunks(4).map(|v| {
        return v.to_vec();
        
    });

    let vec1 = state_1.next().unwrap();
    let vec2 = state_1.next().unwrap();
    let vec3 = state_1.next().unwrap();
    let vec4 = state_1.next().unwrap();

    let mut columns: Vec<Vec<u8>> = vec1.into_iter()
        .zip(vec2.into_iter().zip(vec3.into_iter().zip(vec4.into_iter())))
        .map(|(v1, (v2, (v3, v4)))| {
            return vec![v1, v2, v3, v4];
        }).collect();

    println!("{:?}", columns);

    let mut columns_as_polynomials: Vec<Vec<Polynomial>> = polynomial_conversion(columns);

    println!("{:?}", columns_as_polynomials);


    let matrix_for_mixing: Vec<Vec<Polynomial>> = vec![vec![Polynomial {coeffs: vec![1,1]}, Polynomial {coeffs: vec![1,1,1]}, Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1]}], 
    vec![Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1,1]}, Polynomial {coeffs: vec![1,1,1]}, Polynomial {coeffs: vec![1]}], 
    vec![Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1,1]}, Polynomial {coeffs: vec![1,1,1]}], 
    vec![Polynomial {coeffs: vec![1,1,1]}, Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1]}, Polynomial {coeffs: vec![1,1]}]];

    let irr_poly = Polynomial {coeffs: vec![1,1,1,1,1,1,1,1]};

    columns_as_polynomials.into_iter().map(|column1| {
        matrix_for_mixing.into_iter().map(|mx_row|
            mx_row.into_iter().zip(column1.into_iter()).map(|(a, b)| {
                let premod = a * b;
                let out = 

            }
        )
        )
    });

    let temp_return: Vec<u8> = vec![0];
    temp_return
}

//so how can I work with polynomials in rust without using an external crate
//