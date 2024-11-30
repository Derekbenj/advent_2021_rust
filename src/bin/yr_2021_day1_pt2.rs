use std::{fs::File, io::Read};
use std::iter::zip;

// ownerproof-4281228-1732728071-2018e5ae562c

fn main() {

    let mut str = String::new();
    let _ = File::open("input.txt").expect("i HATE YOU").read_to_string(& mut str);

    let count = count_sum(&str);
    println!("{count}");
}

fn count_sum(strstuff: &str) -> usize {
    let mut iter1 = strstuff.lines().into_iter().map(|x| x.parse::<i32>().expect("Not a number"));
    
    let mut iter2 = iter1.clone();
    iter2.next();
    
    let mut iter3 = iter2.clone();
    iter3.next();

    let mut sum1 = iter1.zip(iter2).zip(iter3).map(|((x, y), z)| x + y + z);
    let mut sum2 = sum1.clone();
    sum2.next();

    let mut count = 0;

    for num in zip(sum1, sum2).map(|(sum1, sum2)| sum2 - sum1) {
        if num > 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    
    #[test]
    pub fn testSum() {
        const TEST: &str = 
"607
618
618
617
647
716
769
792";

        assert_eq!(5, super::count_sum(TEST));
    }
}