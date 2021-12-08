// definite values
enum Move {
    // variants
    Up,
    Down,
    Left,
    Right
}
fn move_char(m: Move){
    // preform action depending on ...
    match m {
        Move::Up => println!("go up"),
        Move::Down => println!("go down"),
        Move::Left => println!("go left"),
        Move::Right => println!("go right"),
    }
}
pub fn stall() {
    //using match
let my_action = Move::Up;
let my_action1 = Move::Right;
let my_action2 = Move::Left;
let my_action3 = Move::Down;
move_char(my_action);
move_char(my_action1);
move_char(my_action2);
move_char(my_action3);
}