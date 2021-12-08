use std::env; // use std env library
pub fn stall(){
let args: Vec<String> = env::args().collect();
// collect arguments in env
let catcher = args[1].clone();
//catch the first argument from vector and make a clone out of it
let status = "100%";
println!("{:?}", catcher);
if catcher == "hello" {
    println!("hello walker, how are you doing ?");
} else if catcher == "status" {
    println!("{}", status);
} else {println!("invalid command");}
}