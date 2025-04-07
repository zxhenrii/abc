
fn main() {
    for number in 1..=10 {
        if number % 2 == 0 {
            println!("{} é par", number);
        } else {
            println!("{} é ímpar", number);
        }
    }
}
