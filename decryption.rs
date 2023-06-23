use std::collections::HashMap;
use crate::polynomials::{binary_to_polynomials, Polynomial, polynomials_to_binary};

pub fn reverse_column_mixing(state: Vec<u8>) -> Vec<u8> {
    let mut state_1 = state.chunks(4).map(|v| {
        v.to_vec()  
    });

    let vec1 = state_1.next().unwrap();
    let vec2 = state_1.next().unwrap();
    let vec3 = state_1.next().unwrap();
    let vec4 = state_1.next().unwrap();

    let columns: Vec<Vec<u8>> = vec1.into_iter()
        .zip(vec2.into_iter().zip(vec3.into_iter().zip(vec4.into_iter())))
        .map(|(v1, (v2, (v3, v4)))| {
            return vec![v1, v2, v3, v4];
        }).collect();

    let columns_as_polynomials: Vec<Vec<Polynomial>> = binary_to_polynomials(columns);

    let matrix_for_mixing: Vec<Vec<Polynomial>> = vec![
    vec![
        Polynomial {poly: vec![3, 2, 1]},
        Polynomial {poly: vec![3, 1, 0]},
        Polynomial {poly: vec![3, 2, 0]},
        Polynomial {poly: vec![3, 0]}], 
    vec![
        Polynomial {poly: vec![3, 0]},
        Polynomial {poly: vec![3, 2, 1]}, 
        Polynomial {poly: vec![3, 1, 0]},
        Polynomial {poly: vec![3, 2, 0]}],
    vec![
        Polynomial {poly: vec![3, 2, 0]},
        Polynomial {poly: vec![3, 0]},
        Polynomial {poly: vec![3, 2, 1]},
        Polynomial {poly: vec![3, 1, 0]}], 
    vec![
        Polynomial {poly: vec![3, 1, 0]},
        Polynomial {poly: vec![3, 2, 0]},
        Polynomial {poly: vec![3, 0]},
        Polynomial {poly: vec![3, 2, 1]}]];

    let mut mixed_columns: Vec<Vec<Polynomial>> = vec![];

    for column in columns_as_polynomials.iter() {
        
        let mut temp_vec = vec![];
        for row in matrix_for_mixing.iter() {
            let mut temp_pol = Polynomial {poly: vec![]};
            for (row_poly, column_poly) in row.iter().zip(column.iter()) {
                    temp_pol = (column_poly.clone() * row_poly.clone()).aes_modulo() - temp_pol;
            }
            temp_vec.push(temp_pol)
        }
        mixed_columns.push(temp_vec)
    }

    let mut result = polynomials_to_binary(mixed_columns);

    let mut final_result = result.into_iter();

    let column1 = final_result.next().unwrap();
    let column2 = final_result.next().unwrap();
    let column3 = final_result.next().unwrap();
    let column4 = final_result.next().unwrap();

    result = column1.into_iter()
    .zip(column2.into_iter().zip(column3.into_iter().zip(column4.into_iter())))
    .map(|(v1, (v2, (v3, v4)))| {
        return vec![v1, v2, v3, v4];
    }).collect();

    result.into_iter().flat_map(|x|x).collect()
}

pub fn reverse_row_shifting(mut state: Vec<u8>) -> Vec<u8> {
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
                    let last_val = val_as_vec.remove(3);
                    val_as_vec.insert(0, last_val);
                    val_as_vec
                }
                3 => {
                    let mut val_as_vec = value.to_vec();
                    let first_two: Vec<u8> = val_as_vec.drain(0..2).collect();
                    val_as_vec.extend(first_two);
                    val_as_vec
                }
                4 => {
                    counter = 0;
                    let mut val_as_vec = value.to_vec();
                    let first_val = val_as_vec.remove(0);
                    val_as_vec.push(first_val);
                    val_as_vec
                }
                _ => value.to_vec(),
            }
        })
        .collect::<Vec<_>>();

    state
}

pub fn create_reverse_s_box_table() -> HashMap<u8, u8> {
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
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
        0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
        0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
        0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
        0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
        0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
        0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
        0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
        0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
        0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
        0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
    ];

    input_byte_vector
        .into_iter()
        .zip(output_byte_vector.into_iter())
        .for_each(|(a, b)| {
            s_box.insert(a, b);
        });
    s_box
}
