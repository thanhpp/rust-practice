use std::io;

fn main() {
    println!("Generate the nth Finonacci number");
    println!("Input n:");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("read n error");

    let n: u32 = n.trim().parse().expect("parse n error");

    match n {
        0 => panic!("invalid n value"),
        1 | 2 => {
            println!("Result");
            println!("1");
        }
        _ => {
            let mut n1 = 1;
            let mut n2 = 1;

            for _ in 2..n {
                let tmp = n1 + n2;
                n2 = n1;
                n1 = tmp;
            }

            println!("Result");
            println!("{}", n1);
        }
    }
}
