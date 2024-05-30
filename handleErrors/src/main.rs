use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
fn main() {
    let greeting_file_res = File::open("hello.txt");

    let greeting_file = match greeting_file_res{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) =>fc,
                Err(e) =>panic!("Error creating the file: {:?}", e),

            },
            other_error =>{
                panic!("Problem opeinig the file: {:?}", other_error)
            }
        }
    };

    // let test_file = File::open("test.txt").unwrap();
    let test2_file =File::open("test2.txt").expect(
        "test2 file should be inlcluded in this project;"
    );

    let test_file = File::open("test.txt")?;

}


// cleaner code

fn bye(){
    let bye_file = File::open("bye.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("bye.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        
    }else {
        panic!("Problem opening the file {:?}", error);
    }});

}

// fn find_username()-> Result<String,io::Error>{
//     let mut username_file =File::open("hello.txt");
//     let mut username = String::new();
//     username_file.read_to_string(&mut username);
//     Ok(username)

// }

