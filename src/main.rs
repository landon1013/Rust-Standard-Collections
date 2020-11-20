
fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    let idx:usize = 0;

    a[idx] = 321;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("Last Elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}
