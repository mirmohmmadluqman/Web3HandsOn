fn main() {
    let gulbab : Person = Person {name: "gulipeer".to_string(), age: 69u8};
    println!("{:?}", Person::new_rand()); // Person
    //// println!("{:?}", gulbab::new_rand()); // # fails because new_rand() doesnt take self reference, 
    // therefore these type of funcs can only be called on Struct Name
    
    ////  println!("{:?}", gulbab.new_rand())  /// ---> will fail too for same reason
    
    println!("{:?}", gulbab.new_rand_self()) 
    
}


#[derive(Debug)]
struct Person {
    name : String,
    age : u8,
}

impl Person {
    fn new_rand() -> Self {
        Person {
            name : "shahabudin".to_string(),
            age : 91u8
        }
    }
    
      fn new_rand_self(&self) -> Self {
        Self {
            name : self.name.clone(), //// if clone() not prsent will fails >>> check data ownership later in rust doc
            age : self.age
        }
    }
}