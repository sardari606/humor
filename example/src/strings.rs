// primtive &str -> immutable, String -> growable, heap allocated structure.
pub fn stall(){
// this is how for strings
    let mut hello = String::from("hello ");
    // how to add to string (need to be mut)
    hello.push('W'); // push method for chars
    hello.push_str("alker"); // push method for string types
println!("{} is the length and {} is the call", hello.len(), hello);
println!("{} is my capacity", hello.capacity()); // showing how much capacity of bytes
println!("{} has the za warudo", hello.is_empty()); //check to see if the val is empty
println!("{} has the za warudo", hello.contains("world")); // does it contain the ("foo") ?
println!("{} , i am here !", hello.replace("Walker", "world")); // replace the ("foo", with "doo")
// looping through the foo by white space
for i in hello.split_whitespace() {
    println!("{}", i);
}
// do yourself a favor and create string with cap
let mut s = String::with_capacity(10);
s.push('a');
s.push('s');
s.push('s');
println!("{} is nice and has a capacity of: {}, {}", s, s.capacity(), s.len());
// assertion
assert_eq!(3, s.len()); // 2 is the value of string
assert_eq!(10, s.capacity()); // 10 is the value of cap that is 10 bytes -> 10 == number of bytes
}