use std::rc::Rc;

fn main() {
    let my_shared = Rc::new(Droppobale::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());
    println!("{my_shared:?}");
    println!("Existing Main...")
}

#[derive(Debug)]
struct Droppobale(i32);

impl Droppobale {
    fn new(n: i32) -> Self {
        println!("Constructing {}", n);
        Self(n)
    }
}

impl Drop for Droppobale {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}

fn move_me(d: Rc<Droppobale>) {
    println!("Moved {}", d.0)
}
