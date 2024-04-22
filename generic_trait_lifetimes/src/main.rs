fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("the largest number: {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("the largest number: {}", largest(&number_list));
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

//  this implementation apply to all generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this implementation only apply for f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
