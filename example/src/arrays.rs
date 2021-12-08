// fixed length same data type
pub fn stall(){
let mut number: [i32; 5] = [1, 2, 3, 4, 5];
println!("{:?} is my array", number); //if not the length you get err also use debug place holder.
println!("{} is the first value", number[0]);
// arrays are mutable
number[0] = 85;
println!("{} is the first value", number[0]);
// you could use .len() for arrays too
// array is stack allocated.
println!("holds this many bytes -> {}", std::mem::size_of_val(&number));
// slice ?!
let slice: &[i32] = &number[1..3];
println!("slice : {:?}", slice); // use debug holder


}
