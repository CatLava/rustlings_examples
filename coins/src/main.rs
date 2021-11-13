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

fn tester(x: Option<i32>) -> Option<i32> {
    match x {
    None => None,
    Some(x) => Some(x+1),
    }
}
fn main() {
    println!("Hello, world!");
    print_coin(Coin::Penny);

    let l1 = Some(3);
    let l2 = tester(l1);
    println!("{:?}", l2 );
    let n = tester(None);
    println!("{:?}", n );


}
