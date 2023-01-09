
// map 実行後は、collect で値を集結させる必要がある。
// これをしないとイテレーターは消費されない
// また、集結させた値を返すので、変数に代入する必要もあり
// https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html
pub fn run() {
    let list = vec!(1,3,4,5);
    println!("list: {:?}", list);

    let list2: Vec<i32> = list.iter().map(|v| v * 2).collect();
    println!("after map: {:?}", list2);
}
