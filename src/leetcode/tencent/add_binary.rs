pub fn add_binary(a: String, b: String) -> String {
    let (mut a, mut b) = (a.chars().rev().peekable(), b.chars().rev().peekable());

    let mut result = vec![];
    let mut jin_wei: u8 = 0;
    while a.peek() != None || b.peek() != None || jin_wei != 0 {
        let mut sum: u8 = 0;
        if let Some(ch) = a.next() {
            sum += char_as_binary(ch);
        }

        if let Some(ch) = b.next() {
            sum += char_as_binary(ch);
        }

        sum += jin_wei;
        jin_wei = sum / 2;
        sum = sum % 2;
        result.push(sum);
    }

    result.iter().rev().map(|b| binary_as_char(*b)).collect::<String>()
}

fn char_as_binary(ch: char) -> u8{
    if ch == '1' {
        1
    } else {
        0
    }
}

fn binary_as_char(b: u8) -> char {
    if b == 1 {
        '1'
    } else {
        '0'
    }
}
