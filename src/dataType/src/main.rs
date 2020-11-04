use std::rc::Rc;
use std::cell::RefCell;

struct S {
    field: i32
}

fn foo(x: Rc<RefCell<S>>) {
    {
        let s = x.borrow();
        println!("the field, twice {} {}", s.field, x.borrow().field);
        // let s = x.borrow_mut(); // Error - we've already borrowed the contents of x
    }

    let mut s = x.borrow_mut(); // OK, the earlier borrows are out of scope
    s.field = 45;
    // println!("The field {}", x.borrow().field); // Error - can't mut and immut borrow
    println!("The field {}", s.field);
}

fn main() {
    let s = S{field:12};
    let x: Rc<RefCell<S>> = Rc::new(RefCell::new(s));
    foo(x.clone());

    println!("The field {}", x.borrow().field);
}