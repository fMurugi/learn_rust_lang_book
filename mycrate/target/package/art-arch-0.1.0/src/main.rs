fn main() {
    println!("Hello, world!");
}

// profiles
// dev
// release

// uploading to crate.io

// documentation starts with three slashes ///

/// Adds one to the number given.
///     
/// # Examples
/// 
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// 
/// ```
/// 

pub fn  add_one(x:i32)->i32{
    x+1
}