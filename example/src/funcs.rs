pub fn stall(){
test("greetings", "hadi"); //parameters in parentheses idk how to spell
let a = add(10, 32); //binded the function to variable
println!("{} is the fn usage", a);
//closure cause idk (it is baste in persian -_-)
 let n3 = 10; //adding to pipe
let x = |n1: i32, n2: i32| n1 +  n2 - n3;
println!("closure is what ? {}", x(4, 5));
}
// new function
// declare types in function's parentheses (called parameters)
fn test(hello: &str, name: &str){
    println!("{} mr {}, nice to meet you", hello, name);
}
fn add(n1: i32, n2: i32) -> i32 {
 n1 + n2 // no semi colon means return some value
}