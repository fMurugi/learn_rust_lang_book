fn main() {
    println!("Hello, world!");
    let mut x =5;
    let y = 6;
    x+=2;
    let z = y+x;
    println!("The value of Z is :{} ",z);

    let tup =('a',7,true);
    let (a,b,c) = tup;
    println!("The value of a is :{} ",a);
    let first =tup.0;
    println!("the value of first is :{}", first);

    // arrays
    let mut arr =[3,7,4];
    arr[1]=5;
    println!("{:?}",arr);

    // functions
    let result = square(5);
    println!("the suare of  {} is {}",5,result);

    // for loop
    for i in 1..11{
        println!("Serving customer number:{}",i);
    }

    // match
    let die1 = 6;
    let die2 = 5;
    match (die1,die2){
        (1,1) => println!("Snake eyes!"),
        (5,5) => println!("You rolled double fives!"),
        (1,_) | (_,1) => println!("You rolled a one!"),
        _ => println!("You did not roll snake eyes or double fives!"),
    }

    // enums and pattern matching
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let localhost:IpAddrKind = IpAddrKind::V4(127,0,0,1);
    


    // Ownership
    let a = String::from("hello");
    say(a.clone());
    print!("{}",a); // this will give an error because the ownership of a has been transferred to the say function

    let s = String::from("cat");
    let plural = pluralize(s.clone());
    println!(
        "I have one{}, you have two {}",
        s,
        plural
    );

    let p_borrowed = pluralize_with_borrow(&s);
    println!(
        "BORROWING - I have one{}, you have two {}",
        s,
        p_borrowed
    );



    // BORROWING
    let p = Person{
        name:String::from("Alice"),
        age:30,
    };

    congratulate(&p);
    println!("{}",p.name); // this will give an error because the ownership of p has been transferred to the congratulate function

    // slice data type
    slice_arr();

    // Borrowing and mutability
    let mut bucket1 = Bucket{litres:5};
    let mut bucket2 = Bucket{litres:10};
    pour(&mut bucket1,&mut bucket2,3);
    println!("{:?}",bucket1);

    // Error handling
    let v = vec![1,20,3];
    v[99];

}

fn square(x:i32) -> i32{
    x*x

}

// enums and pattern matching
enum IpAddrKind{
    V4(u8,u8,u8,u8), 
    V6(String),
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),

}
enum Option<T>{
    Some(T),
    None,
}

impl Message{
    fn some(){
        print!("Hello");
    }
}

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}

enum coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin:coin) -> u32{
    match coin{
        coin::Penny => 1,
        coin::Nickel => 5,
        coin::Dime => 10,
        coin::Quarter => 25
    }
}

// fn plus_one(x:Option<i32>) -> Option<i32> {
//     match x{
//         Some(i) =>println!("The value is :{}",i+1),
//         _ => None,
//     }
// }



//  ownership

fn say(s:String){
    println!("{}",s);

}

fn pluralize(singular:String) -> String{
    singular + "s"
}
// borrowing
fn pluralize_with_borrow(singular:&str) -> String{
    singular.to_owned() + "s"
}

struct Person{
    name:String,
    age:u8,
}

fn congratulate(person:&Person){
    println!("Congratulations, {}!",person.name);
}

// slice data type
fn slice_arr(){
    let a =[4,5,6];
    let slice = &a[1..3];
    println!("{:?}",slice);
}


// Borrowing and mutability

#[derive(Debug)] // what does this mean?
struct Bucket{
    litres:u32,
}

fn pour(source:&mut Bucket, target:&mut Bucket, amount:u32){
    source.litres -= amount;
    target.litres += amount;
}


// return results and ? operator
