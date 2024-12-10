fn main() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    println!("{s}");

    let s2 = String::from("initial contents");
    let hello = String::from("안녕하세요");

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("{s3}");

    let mut s4 = String::from("lo");
    s4.push('l');

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // note s5 has been moved here and can no longer be used

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");

    let s11 = format!("{s8}-{s9}-{s10}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    
    println!("{}", s);

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
