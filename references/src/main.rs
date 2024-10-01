fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    let reference = no_dangle();
    println!("{reference}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");

}

fn no_dangle() -> String {
    let s = String::from("hello 2");

    s
}