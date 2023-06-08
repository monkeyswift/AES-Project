//Isolate the last 4 bytes.
//
//
//
//
//

pub fn rot_word_transformation(mut key: Vec<String>) -> Vec<String> {
    let mut getting_worded: Vec<String> = key.drain(12..16).collect();

    let last_element = getting_worded.pop().unwrap();

    getting_worded.insert(0, last_element);

    key.append(&mut getting_worded);

    key
}
