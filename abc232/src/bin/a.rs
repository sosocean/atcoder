use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let x = s[0].to_digit(10).unwrap();
    let y = s[2].to_digit(10).unwrap();

    println!("{}", x * y);
}
