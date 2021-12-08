// point to a resource in mem -> pointer
pub fn stall(){
// primitive value
let arr1 = [1, 2, 3];
let arr2 = arr1;
println!("this is {:?}", (arr1, arr2));
// because this is primitive it works and no err but if it's non primitive ...arr2
/* if you assign a value to non primitive to a piece of data, the first value will no longer
hold that value, you need to use ref (&) to point to the resource */
let vec1 = vec![1, 2, 3];
let vec2 = &vec1;
/* if vec2 = vec1; println!("this is {:?}", (vec1, vec2)); will get an err cause of owner ship
so point at the resource by using &vec1  */
println!("this is {:?}", (&vec1, vec2));
}