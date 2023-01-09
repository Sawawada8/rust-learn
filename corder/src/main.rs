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
