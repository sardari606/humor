pub fn stall(){
    let age = 20;
    let id: bool = false;
    // normal structure
    // && needs both states
    // || is one condition, also ! is for negative
    if age > 18 && id {
        println!("you are legal");
    } else {
        println!("you are illegal");
   }
   // short if statements
   let truth = if age > 18 {false} else {true};
   println!("truth about my age is {}", truth);
}
