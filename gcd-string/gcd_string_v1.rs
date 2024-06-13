
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let strss: (String, String);
    
    if str1.len() == 0 || str2.len() == 0 {
        return String::new();
    }
    
    // define base-case and long-case
    if str1.len() != str2.len() {
        strss = if str1.len() > str2.len() {
        (str2, str1)
        } else {(str1, str2)};
    } else {
        strss = (str1, str2);
    }
    
    // Check divisor on base-case
    let mut divisor = strss.0.as_bytes().get(0).unwrap();
    
    let mut found_divisor: bool = false;
    let mut ratio: usize = 0;
    
    let base_bytes = strss.0.as_bytes();

    for index in 1..strss.0.len() {
        if Some(divisor) == base_bytes.get(index){
        
            // Check frequency
            if (index+1 < strss.0.len() - 1) 
                && (base_bytes.get(1) != base_bytes.get(index+1))
            {
                continue;
            }
            
            // check divisor is factorial of base-case len
            if strss.0.len() % index == 0  {
                found_divisor = true;
                ratio = index;
            } else { return String::new() }

            break;
        }
    }
    
    println!("ratio : {:?}", ratio);

    //if found symptom of existing of possible GDC in base-case
    let mut gcd: String = String::new();
    
    if found_divisor {
        let possible_divisor = strss.0[..ratio].as_bytes();
        
        // Loop word size of GCD
        for index in 0..(strss.0.len() / ratio) - 1 {
            let base_word = &strss.0[(ratio*(index+1))..ratio*(index+2)];
            
            for (i, byte) in base_word.char_indices() {
                // Verify the GCD
                if possible_divisor[i] ^ byte as u8 != 0 {
                    return String::new()
                }
            }
        }
        
        gcd = unsafe { String::from_utf8_unchecked(possible_divisor.to_vec()) };
    } else if !found_divisor && ratio == 0{
        gcd = strss.0.clone();
        ratio = gcd.len();
    }
    
    // Check long-case size is factorial of GCD.
    if strss.1.len() % ratio != 0 {
        return String::new();
    }
    
    // Verify GCD is divisable to long-case
    let is_really_work: bool = false;
    let gcd_bytes = gcd.as_bytes();
    
    // Loop the long-size word size
    for i in 0..(strss.1.len() / ratio) {
        let long_word = &strss.1[(i* ratio)..(ratio * (i+1))];
        
        for (j, long_char) in long_word.char_indices() {
            // Verify the GCD on long-case
            if (gcd_bytes[j] ^ long_char as u8) != 0 {
                return String::new();
            }
        }
    }

    // Check base-case is the factor of long-case
    if strss.1.len() % strss.0.len() == 0 {
        // Yes
        return strss.0;
    }
    
    if gcd.len() == 1 {
        let mut result = String::new();
        let constant_gcd = gcd.as_bytes().get(0).unwrap();
        for _ in 0..eculid_gcd(strss.0.len(), strss.1.len()) {
            result.push(*constant_gcd as char);
        }
        
        return result;
    }

    gcd
}

pub fn gcd_of_strings_v2(str1: String, str2: String) -> String {
    let stress: (String, String);
    
    if str1.len() == 0 || str2.len() == 0 {
        return String::new();
    }
    
    // define base-case and long-case
    if str1.len() != str2.len() {
        stress = if str1.len() > str2.len() {
        (str2, str1)
        } else {(str1, str2)};
    } else {
        stress = (str1, str2);
    }
    
    // Find proper gcd
    let proper_gcd = eculid_gcd(stress.0.len(), stress.1.len());
    println!("proper_gcd: {:?}", proper_gcd);
    if proper_gcd > stress.0.len() 
        || proper_gcd > stress.1.len() {
        return String::new();
    }
    
    // init base-case and long-case
    let base_bytes = stress.0.as_bytes();
    let long_bytes = stress.1.as_bytes();
    
    let base_word = &base_bytes[..proper_gcd];
    
    if base_bytes.len() % proper_gcd != 0 
        || long_bytes.len() % proper_gcd != 0{
        return String::new();
    }
    
    // Verify on base-case
    let base_ratio = base_bytes.len() / proper_gcd;
    for i in 1..base_ratio {
    
        let sub_base_bytes_list = &base_bytes[proper_gcd*i..proper_gcd*(i+1)];
        
        for (index, sub_byte) in sub_base_bytes_list.iter().enumerate() {
            if base_word[index] ^ sub_byte != 0 { return String::new(); }
        }
    }
    
    // Verify on long-case
    let long_ration = long_bytes.len() / proper_gcd;
    for i in 0..long_ration {
    
        let sub_base_bytes_list = &long_bytes[proper_gcd*i..proper_gcd*(i+1)];
        
        for (index, sub_byte) in sub_base_bytes_list.iter().enumerate() {
            if base_word[index] ^ sub_byte != 0 { return String::new(); }
        }
    }
    
    let mut gcd_string = String::new();
    gcd_string = unsafe { String::from_utf8_unchecked(base_word.to_vec()) };
    
    gcd_string
}

fn eculid_gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return eculid_gcd(b, a % b)
    }
}

fn main() {
    let str1 = String::from("LEET");
    let str2 = String::from("CODE");
    
    println!("str1: {:?}", str1);
    println!("len: {:?}", str1.len());
    
    println!("str2: {:?}", str2);
    println!("len: {:?}", str2.len());
    
    let gcd = gcd_of_strings_v2(str1, str2);
    println!("GCD: {:?}", gcd);
    
    
}