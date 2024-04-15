fn main() {
    let scale = 2;
    let rect1 = (32, 32);
    let rect2 = Rectangle {
        width: dbg!(rect1.0 * scale), // dbg! return the ownership so width will be valid
        height: rect1.1,
    };

    println!("{:#?}", rect2); // :? for debug print, add # to format the output
    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect2)
    );

    dbg!(&rect2); // debug print
}

// itter 1
fn area_1(w: u32, h: u32) -> u32 {
    w * h
}

// itter2
fn area_2(dimensions: (u32, u32)) -> u32 {
    area_1(dimensions.0, dimensions.1)
}

// itter 3
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_3(rect: &Rectangle) -> u32 {
    area_2((rect.width, rect.height))
}
