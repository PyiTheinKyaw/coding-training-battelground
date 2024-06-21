fn main() {
    // let letters = vec!['c','f','j'];
    let letters = vec!['e','e','e','e','e','e','n','n','n','n'];
    let target = 'e';

    let result = next_greatest_letter(letters, target);
    println!("Result: {:?}", result);
}

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    use std::cmp::Ordering;

    if !(letters.len() >= 2 && letters.len() <= 10i32.pow(4) as usize) { return letters[0] }
    if !target.is_ascii_lowercase() || target == 'z' { return letters[0] }

    //Apply bineary search here
    let mut left = 0;
    let mut right = letters.len();

    let mut greater_char_index = 0;

    while left < right {

        let mid = left + (right - left) / 2;

        if !letters[mid].is_ascii_lowercase() { return letters[0] }

        match letters[mid].cmp(&target) {
            Ordering::Equal => {
                let mut _found_index = mid % letters.len();

                while _found_index+1 < letters.len() {
                    if letters[_found_index] != letters[_found_index + 1] {
                        greater_char_index = (_found_index + 1) % letters.len();
                        break;
                    }
                    _found_index += 1;
                }

                break;
            },
            Ordering::Greater => right = mid,
            Ordering::Less => left = mid + 1 
        }

        if left == right {
            greater_char_index = left % letters.len();   
            break;
        }
    }

    letters[greater_char_index]
}