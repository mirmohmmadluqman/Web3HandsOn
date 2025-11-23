fn main() {
    println!("-----------------------------------------------------------------------------------");

    let gulbab = Person::new("gulipeer".to_string(), 69);
    println!("{:?}", gulbab);

    println!("{:?}", Person::new_rand());
    println!("{:?}", gulbab.new_rand_self());

    println!("-----------------------------------------------------------------------------------");

    let person1 = LGBTQ::real_one();
    let person2 = person1.also_real();

    println!("{:?}", person1);
    println!("{:?}", person2);

    println!("-----------------------------------------------------------------------------------");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn new_rand() -> Self {
        Self {
            name: "shahabudin".to_string(),
            age: 91,
        }
    }

    fn new_rand_self(&self) -> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RGENDER {
    Others, // ← only keep the one you actually use
}

#[derive(Debug)]
struct LGBTQ {
    name: String,
    real_gender: RGENDER,
    age: u8,
}

impl LGBTQ {
    fn real_one() -> Self {
        Self {
            name: "Ibraheem Rashid Khaja".to_string(),
            real_gender: RGENDER::Others,
            age: 16,
        }
    }

    fn also_real(&self) -> Self {
        Self {
            name: self.name.clone(),
            real_gender: self.real_gender,
            age: self.age,
        }
    }
}
