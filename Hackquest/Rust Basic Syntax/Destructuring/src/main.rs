struct Ferris {
    e: i32,
    f: String
}

fn main() {
    
    println!("Destructuring in Rust:");
    let (a, b, c, d, e, f);
    // println!("{}{}{}{}{}{}", a, b, c, d, e, f); // just to avoid unused variable warning

    // tuple destructure
    (a, b) = (1, 2);
    println!("a: {}, b: {}", a, b);

    // Array destructuring, .. means ignoring multiple elements, _ means ignoring the element at the corresponding index position (1)
    [c, .., d, _] = [1, 2, 3, 4, 5];
    println!("c: {}, d: {}", c, d);

    // struct destructure
    Ferris { e, f } = Ferris { e:5, f:"rust".to_string() };
    println!("e: {}, f: {}", e, f);

    assert_eq!((1, 2, 1, 4, 5, "rust".to_string()), (a, b, c, d, e, f));

    println!("Destructuring completed successfully!," );


    println!("Quest 1/1: Please change the __ in assert_eq!(x, __); and assert_eq!(y, __); to the correct value and fill in the following lines of code"); 
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];

    // assert_eq!(x, __);    
    assert_eq!(x, 3);
    println!(" = {}", 3);

    // assert_eq!(y, __);
    assert_eq!(y, 2);
    println!("y = {}", 2);

}