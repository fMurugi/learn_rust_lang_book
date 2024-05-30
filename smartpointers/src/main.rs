

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("{:?}", list);

let x =5;
let y = &x;
let z = Box::new(x);
let w = MyBox::new(x);
assert_eq!(x,5);
assert_eq!(*y,5);
assert_eq!(*z,5);
assert_eq!(*w,5);

let m = MyBox::new(String::from("Rust"));
hello(&m);

// DRop trait
let d = CustomSmartPointer{data: String::from("my stuff")};
let e = CustomSmartPointer{data: String::from("other stuff")};
println!("CustomSmartPointers created.");


}

fn hello(name:&str){
    println!("Hello, {}", name);




}


struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

// Deref trait returns reference to the value stored in the Box
// it does not return the value because it would take ownership of the value
// 
impl<T> std::ops::Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

// Deref and Drop trait
// smart pointers implement the Deref and Drop traits
// when to use Box<T> to store data on the heap
// Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> to be treated like a reference
// Box<T> also implements the Drop trait, which allows it to clean up the heap data when a Box<T> goes out of scope

// deref corecsion is a convenience that Rust performs on arguments to functions and methods

// when rust does deref coersion


// DRop Trait
// Drop trait lets you customize what happens when a value is about to go out of scope
// Drop trait requires you to implement a method called drop that takes a mutable reference to self
// Rust calls the drop method automatically when a value goes out of scope
//  example of using this trait is to clean up resources like files or network connections



struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// reference counting smart pointer
// Rc<T> is a reference counting smart pointer
// Rc<T> keeps track of the number of references to a value which determines whether or not a value is still in use
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid


// interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
// to mutate data, the pattern uses unsafe code inside a data structure to bend Rust's usual rules that govern mutation and borrowing
// unsafe code is code that is not checked during compile time
// Rust's standard library has a number of types that use interior mutability
// RefCell<T> is a type that enforces the borrowing rules at runtime instead of compile time
// RefCell<T> keeps track of how many references are currently active
// If there are any active mutable references, then any attempt to borrow the value immutably will result in a panic


// the halting problem?
// the halting problem is a famous problem in computer science that is impossible to solve
// the halting problem asks if it is possible to write a program that can determine whether any other program will halt or run forever


// the borrow checker is a feature of the Rust compiler that enforces these rules
// the borrow checker prevents data 


// recap of how to choose which smart pointer to use
// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable

