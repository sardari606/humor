pub fn stall(){
    let mut x = 0;
    //bioshock infinite looper
    loop {
        x += 1;
        println!("{} is over", x);
        if x >= 0 {break;}
    }
    let mut  i = 0;
    // while looper (FizzBuzz)
    while i <= 100 {
        if  i % 5 == 0 && i % 3 == 0 { // you could've type % 15 -_-
            println!("fizzbuzz! {}", i);
        } else if i % 5 == 0 {
            println!("buzz! {}", i);
        } else if i % 3 == 0 {
            println!("fizz! {}", i);
        } else {
            println!("thanks {}", i);
        }
        i += 1;
   }
   // for range !
   for i in 0..100 {
    if  i % 5 == 0 && i % 3 == 0 { // you could've type % 15 -_-
    println!("fizzbuzz! {}", i);
} else if i % 5 == 0 {
    println!("buzz! {}", i);
} else if i % 3 == 0 {
    println!("fizz! {}", i);
} else {
    println!("thanks {}", i);
}
   }
}