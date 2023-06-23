
pub fn rot_word_transformation(mut key: Vec<u8>) -> Vec<u8> {
    let tempval = key.remove(0);
    key.push(tempval);
    key
}

pub fn round_constants_generator(mut last_constant: Vec<u8>) -> () {
    if last_constant[0] < 128 {
        last_constant[0] = last_constant[0] * 0x02
    } else {
        let tempval = (last_constant[0] * 0x02) as u16 ^ 0x11B;
        last_constant[0] = tempval as u8;
    }
}
