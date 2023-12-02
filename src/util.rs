use std::fs;

pub fn read_input(day: usize) -> String {
    let path = if day > 9 {
        format!("input/day{}.txt", day)
    } else {
        format!("input/day0{}.txt", day)
    };

    match fs::read_to_string(&path) {
        Ok(s) => return s,
        Err(e) => panic!("ERROR: reading file {path} was not possible. {e}"),
    }
}
