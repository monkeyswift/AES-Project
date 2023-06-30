use std::collections::HashMap;

pub fn rot_word_transformation(mut key: Vec<u8>) -> Vec<u8> {
    let tempval = key.remove(0);
    key.push(tempval);
    key
}

pub fn round_constants(mut last_constant: Vec<u8>) -> Vec<u8> {
    if last_constant[0] < 128 {
        last_constant[0] = last_constant[0] * 0x02 as u8
    } else {
        let tempval = (last_constant[0] as u16 * 0x02 as u16) ^ 0x11b;
        last_constant[0] = tempval as u8;
    }
    last_constant
}

pub fn generate_round_keys(key: u128, s_box: HashMap<u8, u8>) -> Vec<Vec<u8>> {

    let mut key_vector: Vec<Vec<u8>> = key.to_ne_bytes().chunks(4).map(|x|x.to_vec()).collect();

    let mut vector_for_key_addition_layers: Vec<Vec<u8>> = vec![];

    for _ in 0..11 {

        let mut key_vector_iter = key_vector.clone().into_iter();

        let key_vec1 = key_vector_iter.next().unwrap();
        let key_vec2 = key_vector_iter.next().unwrap();
        let key_vec3 = key_vector_iter.next().unwrap();
        let key_vec4 = key_vector_iter.next().unwrap();

        let mut travler = rot_word_transformation(key_vec4.clone());

        travler = travler.iter().map(|value| *s_box.get(value).unwrap()).collect();
        
        travler = round_constants(travler);

        let mut modded_key_vec1: Vec<u8> = key_vec1.iter().zip(travler.iter()).map(|(unit1, unit2)| {
            let result = *unit1 ^ *unit2;
            result
        }).collect();

        let modded_key_vec2: Vec<u8> = modded_key_vec1.iter().zip(key_vec2.iter()).map(|(unit1, unit2)| {
            let result = *unit1 ^ *unit2;
            result
        }).collect();

        let modded_key_vec3: Vec<u8> = modded_key_vec2.iter().zip(key_vec3.iter()).map(|(unit1, unit2)| {
            let result: u8 = *unit1 ^ *unit2;
            result
        }).collect();

        let modded_key_vec4: Vec<u8> = modded_key_vec3.iter().zip(key_vec4.iter()).map(|(unit1, unit2)| {
            let result: u8 = *unit1 ^ *unit2;
            result
        }).collect();

        let temp_vec = vec![modded_key_vec1.clone(), modded_key_vec2.clone(), modded_key_vec3.clone(), modded_key_vec4.clone()];

        modded_key_vec1.extend(modded_key_vec2);

        modded_key_vec1.extend(modded_key_vec3);

        modded_key_vec1.extend(modded_key_vec4);

        vector_for_key_addition_layers.push(modded_key_vec1);

        key_vector = temp_vec;

    }
    vector_for_key_addition_layers
}
