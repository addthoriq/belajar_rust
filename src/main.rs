use std::io;


fn main() {
    println!("Masukkan Angka: ");
    let mut nomor = String::new();
    io::stdin().read_line(&mut nomor).expect("Failed");
    fizzbuuz(nomor);
}

fn fizzbuuz(nomor: String){
    let num_conv: i32 = nomor.trim_end().parse().unwrap();
    for i in 1 .. num_conv{
        let can_div_3: bool = i % 3 == 0;
        let can_div_5: bool = i % 5 == 0;

        if can_div_3 && can_div_5 {
            println!("FizzBuzz");
        }
        else if can_div_3 {
            println!("Fizz")
        }
        else if can_div_5{
            println!("Buzz")
        }
        else {
            println!("{}", i)
        }
    }
}