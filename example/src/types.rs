/* u8, i8, u16, i16, u32, i32(default), u64, i64, u128, i128 (bits they take in memory)
f32, f64
Boolean
char
(tuple, arrays)
*/
//statically lang -> know the types
pub fn stall() {
//def i32
    let x = 2;
    // def is f64
    let y = 2.5;
    //full var
    let z: i64= 45454545;
    println!("max for i32 is {}", std::i32::MAX);
    println!("max for i64 is {}", std::i64::MAX);
 //bool
    let beep = true;
 let bep: bool= 19>10;
 //char
 let a1 = 'a';
 //unicode
 let a2 = '\u{1F602}';
 println!("{:?}", (x, y, z, beep, bep, a1, a2));
}