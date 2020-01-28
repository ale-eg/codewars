//https://www.codewars.com/kata/526571aae218b8ee490006f4
#![allow(unused_parens)]
fn count_bits(mut n: i64) -> u32 {
    let mut res: u32 = 0;
    while (n > 0){
        res = res + 1;
        n &= n - 1;
    }
    return res;
}

fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}

fn main(){
    returns_expected();
}
