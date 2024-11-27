use std::{fs::File, io::Read};
use std::iter::zip;

// ownerproof-4281228-1732728071-2018e5ae562c

fn main() {

    let mut str = String::new();
    let _ = File::open("input.txt").expect("i HATE YOU").read_to_string(& mut str);

    let count = count_increasing(&str);
    println!("{count}");
}

fn count_increasing(strstuff: &str) -> usize {


    let mut iter1 = strstuff.lines().into_iter();
    let mut iter2 = strstuff.lines().into_iter();
    iter1.next();
    let mut both = zip(iter1, iter2);

    let mut count = 0;

    for (one, two) in both {
        if one.trim().parse::<i32>().expect("") > two.trim().parse::<i32>().expect("") {
            count += 1;
        }
    }

    count as usize
}

#[cfg(test)]
mod tests {

    const PROCESS: usize = 7;

    #[test]
    pub fn testInput() {
        const TEST: &str = 
"199 
200
208
210
200
207
240
269
260
263";

        assert_eq!(7, super::count_increasing(TEST));
    }
}