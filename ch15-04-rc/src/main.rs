use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))) ;
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
}

enum List{
    Cons(i32, Rc<List>),
    Nil,
}
