<!-- https://chatgpt.com/g/g-p-68d27c09e5748191b46db5318f9494d8-svm/c/691b1dae-2384-8320-9005-7236770b7ded: -->
# 1️⃣ **Vector of floats (Vec<f64>) → only floats**

```rust
fn main() {
    let mut v: Vec<f64> = Vec::new();
    v.push(3.14);
    v.push(2.718);

    println!("{:?}", v);  // [3.14, 2.718]
}
```

❌ Cannot push:
`v.push(1)` → compile error (i32)
`v.push("hi")` → compile error
`v.push(())` → compile error

---

# 2️⃣ **Vector of integers**

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);

    println!("{:?}", v);  // [10, 20]
}
```

---

# 3️⃣ **Vector of Strings**

```rust
fn main() {
    let mut names: Vec<String> = Vec::new();

    names.push("hello".to_string());
    names.push("world".to_string());

    println!("{:?}", names);   // ["hello", "world"]
}
```

---

# 4️⃣ **Vector of Unit type `Vec<()>`**

Yes, you can make a vector of unit values:

```rust
fn main() {
    let mut v: Vec<()> = Vec::new();
    v.push(());   // only () allowed
    v.push(());

    println!("{:?}", v);  // [(), ()]
}
```

---

# 5️⃣ **Vector of Structs**

```rust
struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut users: Vec<User> = Vec::new();

    users.push(User { id: 1, name: "Alice".to_string() });
    users.push(User { id: 2, name: "Bob".to_string() });

    println!("Users: {}", users.len()); // 2
}
```

---

# 6️⃣ **Vector of Arrays**

```rust
fn main() {
    let mut v: Vec<[i32; 3]> = Vec::new();

    v.push([1, 2, 3]);
    v.push([4, 5, 6]);

    println!("{:?}", v);  // [[1,2,3], [4,5,6]]
}
```

❗ Notice:
You must push **same-sized arrays**.

---

# 7️⃣ **Vector of Tuples** (mix types *inside* tuple)

If you want different types, use tuples:

```rust
fn main() {
    let mut v: Vec<(i32, f64, String)> = Vec::new();

    v.push((10, 3.14, "hi".to_string()));
    v.push((20, 2.71, "bye".to_string()));

    println!("{:?}", v);
}
```

Here:

* The vector type is 1 type: **tuple**
* Inside tuple you can mix types safely

---

# 8️⃣ **Vector of Enums** (best way to store mixed data)

```rust
enum Data {
    Number(i32),
    Text(String),
    Nothing,
}

fn main() {
    let mut v: Vec<Data> = Vec::new();

    v.push(Data::Number(100));
    v.push(Data::Text("hello".to_string()));
    v.push(Data::Nothing);

    println!("Stored {} items", v.len());
}
```

This is how Rust stores different types in one vector **safely**.

---

# 🧠 **Final Summary**

### ✔ A vector has one type

### ❌ You cannot push different types

### ✔ You CAN push:

* floats → into Vec<f64>
* ints → into Vec<i32>
* strings → into Vec<String>
* structs → into Vec<MyStruct>
* arrays → into Vec<[i32; 3]>
* unit → into Vec<()>

### ✔ For different types inside one vector use:

* tuples
* enums
* trait objects
