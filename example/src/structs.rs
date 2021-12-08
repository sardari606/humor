// custom data types
//traditional struct, out of fn !
struct Rgb {
    // proprety
    red: u8,
     //green: u8,
     blue: u8
 }
 // there are tuple structs
 struct Red(u8, u8, u8);
struct Person {
    first: String,
    last: String
}
impl Person {
// construct Person
    fn muse(fname: &str, lname: &str) -> Person {
        Person{
            first: fname.to_string(),
            last: lname.to_string()
        }
    }
    // get full name
    fn get_name(&self) -> String {
        format!("{}, {}", self.first, self.last)
    }
    // change the last name
    fn set_lname(&mut self, lastn: &str) {
        self.last = lastn.to_string();
    }
    //tuple it
    fn tupler(self) -> (String, String) {
        (self.first, self.last)
    }
}
pub fn stall(){
 let mut c = Rgb {
     red: 255,
     //green: 0,
     blue: 0,
 };
 c.red = 128;
 let b = Red(27, 58, 69);
 println!("roses are {}, {}, {}", c.red, c.blue, b.2);
 //impl usage
 let mut p = Person::muse("hadi", "sardari");
 println!("hello {}", p.get_name());
 p.set_lname("gordy");
 println!("hello {}", p.get_name());
 println!("hello {:?}", p.tupler());
}