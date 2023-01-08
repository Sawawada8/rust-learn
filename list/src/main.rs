
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}

fn main() {
    println!("---------------");

    let b = Box::new(
        List::Node(3, Box::new(
            List::Node(4343, Box::new(
                List::Node(55, Box::new(
                    List::Node(12, Box::new(
                        List::Nil
                    )))))))));
    let list = List::Node(43, b);

    println!("{:?}", list);

    rec(list);

    println!("---------------");
}

// 初回は List そのものを受け付けて、
// rec2 では、 Box を受け付けて再帰的に実行させる
fn rec(ori: List) {
    match ori {
        List::Node(val, list) => {
            println!("First: val: {}, list: {:?}", val, list);
            rec2(list);
        },
        List::Nil => {
            println!("First: nillllll");
        }
    }
}

// 二回目以降はbox受け取る
fn rec2(ori: Box<List>) {
    // Box はポインタなので、* で値を取得している
    match *ori {
        List::Node(val, list) => {
            println!("val: {}, list: {:?}", val, list);
            rec2(list);
        },
        List::Nil => {
            println!("nillllll");
        }
    }
}

#[test]
fn demo() {
    assert_eq!(3, 3);
}
