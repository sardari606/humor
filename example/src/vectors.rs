// resizeable arrays
pub fn stall(){
    let mut number: Vec<i32> = vec![1, 2, 3, 4, 5];
    // add on to vec
    number.push(69);
    number.push(70);
    number.pop(); // last index will be gone
    println!("{:?} is my array", number); //if not the length you get err also use debug place holder.
    println!("{} is the first value", number[0]);
    // arrays are mutable
    number[0] = 85; // re assign val
    println!("{} is the first value", number[0]);
    // you could use .len() for vecs too
    // vectors is stack allocated.
    println!("holds this many bytes -> {}", std::mem::size_of_val(&number));
    // slice ?!
    let slice: &[i32] = &number[1..3];
    println!("slice : {:?}", slice); // use debug holder
    // we have something called pop off
    // loop through the vec
    for x in number.iter() {
        println!("{}", x);
    }
    for x in number.iter_mut() {
        *x *= 2; // each multiply by 2 like map
    }
    println!("{:?}", number);
    }