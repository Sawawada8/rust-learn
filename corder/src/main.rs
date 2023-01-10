use std::slice::Iter;

fn main() {
    rec();

    let data = vec!(
        vec!('#', '#', '#', '#'),
        vec!('#', '#', 'S', '#'),
        vec!('#', '.', '.', '#'),
        vec!('.', '.', '#', '#'),
    );
    let lact = get_start(&data);
    // let a = data[0][888];
    println!("{:?}, {}",
        lact,
        data[lact.0][lact.1]);
}

fn get_start(data: &Vec<Vec<char>>) -> (usize, usize) {
    let mut lact = (0, 0);

    for ( i, v ) in data.iter().enumerate() {
        // println!("auter: {:?}, {}", v,i);
        for ( ii, vv ) in v.iter().enumerate() {
            // println!("inner: {:?}, {}", vv, ii);
            if *vv == 'S' {
                // println!("S");
                lact = (i,ii);
                break;
            }
        }
    }

    println!("get_start: {:?}", lact);
    lact
}

enum List {
    Node(i32, Box<List>),
    Nil,
}

fn rec_main() {
    let first = 3;
    let l = rec_s(first);
}

fn rec_s(first: i32) -> List {
    List::Node(first, Box::new(List::Nil))
}

fn rec_list(l: List, i: i32) -> List {
    List::Node(i, Box::new(l))
}
#[test]
fn rec_list_test() {
    let fir = 3;
    let sec = 33;
    let l = rec_s(fir);
    let ll = rec_list(l, sec);

    let expected = List::Node(33, Box::new(
        List::Node(3, Box::new(
            List::Nil))));

    // ここ値入れて初期化すれば通った
    // 型定義のみだとだめだった。。
    let mut inner: List = List::Nil;
    let a = match expected {
        List::Node(i, l) => {
            inner = *l;
            i
        },
        List::Nil => { 0 }
    };
    assert_eq!(a, 33);

    let b = match inner {
        List::Node(i, _l) => {
            i
        },
        List::Nil => { 0 }
    };
    assert_eq!(b, 3);
}

fn rec() {
    let list = vec!(1,3,4,5,2,9);
    let mut list = list.iter();

    a(&mut list);

    println!("0: {:?}", list);
    println!("1: {:?}", list.next());
    println!("2: {:?}", list);
}

fn a(list: &mut Iter<i32>) {
    println!("{:?}", list);
    // count() は消費されてしまうので、clone を作っって数取得している
    let count = list.clone().count();
    println!("count: {}",count);
    println!("{:?}", list);
    if count < 2 {
        println!("----____ complete!! ----____");
        return;
    }

    // susumeru
    let val = list.next().unwrap();
    println!("unwrapval: {}", val);
    a(list);
}

#[test]
fn get_start_test() {
    let d = vec!(
        vec!('#','#', '.','S')
    );
    let posi = get_start(&d);

    assert_eq!((0,3),posi);
}
