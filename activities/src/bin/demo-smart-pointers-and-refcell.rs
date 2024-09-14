#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

struct Packages {
    quantity: u32,
}

type SharedFreight = Rc<RefCell<Packages>>;

struct Dispatch(Rc<RefCell<Packages>>);
struct Truck(Rc<RefCell<Packages>>);

fn main() {
    let package = Rc::new(RefCell::new(Packages { quantity: 5 }));

    let dispatch = Dispatch(Rc::clone(&package));
    let truck = Dispatch(Rc::clone(&package));

    assert_eq!(truck.0.borrow().quantity, 5);

    {
        let mut freight = truck.0.borrow_mut();
        freight.quantity -= 1;
    }
    assert_eq!(truck.0.borrow().quantity, 4);
    assert_eq!(dispatch.0.borrow().quantity, 4);
}
