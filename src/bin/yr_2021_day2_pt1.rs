use std::fs::File;
use std::io::Read;
use std::vec;

fn main() {
    let mut str = String::new();
    let _ = File::open("input_day_2.txt")
        .expect("i HATE YOU")
        .read_to_string(&mut str);

    let mut h = 0;
    let mut v = 0;

    (h, v) = counts(&str);

    println!("{0}", h * v)
}

fn counts(text: &str) -> (i32, i32) {

    let mut horizontal = 0;
    let mut vertical = 0; 

    let lines = text.lines().into_iter();
    for line in lines.map(|s| s.split_whitespace().collect::<Vec<_>>()) {
        if line.len() == 2 {
            let change: i32 = line.get(1).unwrap().parse().unwrap();
            if let Some(&x) = line.get(0) {
                match x {
                    "forward" => horizontal += change,
                    "down" => vertical += change,
                    "up" => vertical -= change,
                    _ => panic!("Unknown Word"),
                }
            }
        }
    }

    (horizontal, vertical)
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn testCount() {
        const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

        let mut h = 0;
        let mut v = 0;

        (h, v) = super::counts(INPUT);

        assert_eq!(h * v, 150);
    }
}
