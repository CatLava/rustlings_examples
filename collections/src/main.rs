use std::collections::HashMap;

fn string_stuff(ss: &str)  {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{} is {} long", s, s.len());
    s.push_str(ss)
}

fn hashes() {
    let mut scores = HashMap::new();

    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("blue"), 30);

    for (i,j) in &scores{
        println!("{}, {}", i,j );
    }

    // now to overwrite a values

    scores.insert(String::from("blue"), 33);
    println!("{:?}", scores)
}
fn main() {

    let mut v= vec![7,5,4];

    v.push(8);

    println!("Hello, world! {:?}", v);

    for i in &mut v{
        *i += 10;
        println!("{}", i)
    }
    string_stuff("meh");
    hashes()

}
