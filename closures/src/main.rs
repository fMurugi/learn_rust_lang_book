use std::ptr::eq;

fn main() {
    let x = 4;
    // fn equal_to_x(z:i32)->bool{
    //     z==x
    // }

    let equal_to_x = |z| z == x;

    let y = 4;
    println!("{}",equal_to_x(y));
    assert!(equal_to_x(y));


    let x = vec![1,2,3];
    let x_it = x.iter();
    for i in x_it{
        println!("{}",i);

    }

    // adapter - map, filter, collect, takes in an iterator and returns an iterator
    //consumer - sum, takes in an iterator and returns a single value eg interger, collection
    

}

#[test]
fn iterartor_demo(){
    let x = vec![1,2,3];
    let mut x_it = x.iter();
    
    assert_eq!(x_it.next(),Some(&1));
    assert_eq!(x_it.next(),Some(&2));
    assert_eq!(x_it.next(),Some(&3));
    assert_eq!(x_it.next(),None);

}

#[test]
fn iterartor_demo2(){
    let x = vec![1,2,3];
    let  x_it = x.iter();
    let total:i32 = x_it.sum();
    assert_eq!(total,6);
}

// using adatpter map and consumer collect
#[test]
fn iterartor_demo3(){
    let x = vec![1,2,3];
    let  x_it = x.iter();
    let res: Vec<_> = x_it.map(|x| x+1).collect();
}



    // closures capture environment in three ways
    // 1. taking ownership
    // 2. borrowing
    // 3. mutable borrowing
    // fnOnce, fnMut, fn


 
