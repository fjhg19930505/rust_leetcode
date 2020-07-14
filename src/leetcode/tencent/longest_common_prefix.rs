pub fn longest_common_prefix(strs: Vec<String>) -> String {
    
    let mut shorest: usize = 0;
    for s in strs.iter() {
        let len = s.len();
        if len == 0 {
            return String::from("");
        }
        if shorest == 0 || shorest > len{
            shorest = len;
        }
    }

    let mut temp_str = "";
    println!("shortest is {}", shorest);
    let mut i : i32 = shorest as i32 - 1;
    while i >= 0 {
        println!("i is {}", i);
        for s in strs.iter() {
           println!("char is {}", s);
            if temp_str.is_empty() {
                temp_str = &s[0 as usize..(i+1) as usize];
            }

            if &s[0 as usize..(i+1) as usize] != temp_str {
                temp_str = "";
                break;
            }
        }
        if !temp_str.is_empty() {
            return String::from(temp_str);
        }
        i = i - 1;
    }

    return String::from(temp_str);
}