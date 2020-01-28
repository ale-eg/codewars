//https://www.codewars.com/kata/526571aae218b8ee490006f4
fn count_bits(n: i64) -> u32 {
    // code here
    let mut m = n;
    let mut t = 0;
    let mut over:u32 = 0;
    loop{
        t = m / 2;
        if (m - t*2 ) == 1{
            over += 1;
        }
        m = t;
        if t < 1{
            break
        }
    }
    println!("over:{}", over);
    return over
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