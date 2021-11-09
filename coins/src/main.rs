enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn print_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { println!("Pennywise");
        1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    println!("Hello, world!");
    print_coin(Coin::Penny);
}
