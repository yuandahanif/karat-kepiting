use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    let print_list = || println!("{:?}", list);

    print_list();

    let mut list = list;
    let mut push_to_list = || list.push(100);
    //  cant use list between line 8 to 11
    // println!("{:?}", list);
    push_to_list();

    println!("{:?}", list);

    println!("before move ownership of list: {:?}", list);

    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();

    // list is no longer with us 🙏🙏🙏
}
