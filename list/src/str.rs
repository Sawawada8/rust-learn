pub fn run() {
    // bool などは、スタックで管理されるので、所有権などのことは考えなくていい
    // String 等の場合は、heap にほぞんされるので、参照させる必要があったりする
    let b = true;
    hoi(b);
    println!("{}", b);

    let s = String::from("hello");
    hoihoi(&s);
    println!("{}", s);

    // そのまま渡しちゃうが、return で返せば一応問題なく動作できる
    // けど基本は、参照で渡す感じがめんどくさくなくていい！データ返さないといけない場合、いちいちtuple で返したりしなかったりするので、、
    let mut s = String::from("hello");
    s = hoihoi2(s);
    println!("{}", s);

}

fn hoi(i: bool) {
    println!("bool {}", i);
}
fn hoihoi(i: &String) {
    // そのままでも見れるし、参照外ししても見れる
    println!("String {}, {}", i, *i);
}
fn hoihoi2(i: String) -> String {
    println!("String {}", i);
    i
}

