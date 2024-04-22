fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("the largest number: {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("the largest number: {}", largest(&number_list));
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
