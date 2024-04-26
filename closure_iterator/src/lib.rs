mod shoe {
    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String,
    }

    pub fn shoe_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|shoe| shoe.size == size).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            shoe::Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            shoe::Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            shoe::Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoe::shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                shoe::Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                shoe::Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
