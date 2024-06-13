impl Solution {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let version1 = format!("{}{}", str1, str2);
        let version2 = format!("{}{}", str2, str1);
        if version1 != version2 {
            return "".to_string();
        }
        let g = Self::gcd(str1.len() as u32, str2.len() as u32);
        str1.get(0..(g as usize)).unwrap().to_string()
    }
}