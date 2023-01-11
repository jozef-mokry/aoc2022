fn main() {
    let mut result = String::from("0");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        result = add(result, line);
    }
    println!("{result}");
}

fn add(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut i = 0;
    while i < a.len() || i < b.len() || carry != 0 {
        let sum = carry + to_digit(a.bytes().nth_back(i)) + to_digit(b.bytes().nth_back(i));
        let c;
        (carry, c) = match sum {
            -5..=-3 => (-1, to_char(sum + 5)),
            -2..=2 => (0, to_char(sum)),
            3..=5 => (1, to_char(sum - 5)),
            x => unreachable!("{x}"),
        };
        result.push(c);
        i += 1;
    }
    unsafe {
        // SAFETY: All bytes are ASCII
        result.as_bytes_mut().reverse();
    }
    result
}

fn to_digit(d: Option<u8>) -> i8 {
    match d {
        None => 0,
        Some(b'=') => -2,
        Some(b'-') => -1,
        Some(x @ b'0'..=b'2') => (x - b'0') as i8,
        Some(_) => unreachable!(),
    }
}

fn to_char(c: i8) -> char {
    match c {
        -2 => '=',
        -1 => '-',
        x @ 0..=2 => (x as u8 + b'0') as char,
        _ => unreachable!("{c}"),
    }
}
