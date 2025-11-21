use std::{
    cell::{Cell, Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

fn main() {
    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let z = MyBox::new(x);
    // assert_eq!(5, *z);

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    x.set(3);
    x.set(4);
    println!("{}", x.get());

    let x = RefCell::new(String::from("hi"));
    let mut y = x.borrow_mut();
    println!("{:?}", x);

    let choi = Person::new();

    // println!("{:?}", choi);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct Person<'a> {
    count: u8,
    name: RefCell<&'a str>,
}

impl<'a> Person<'a> {
    fn new() -> Self {
        Self {
            count: 0,
            name: RefCell::new("default"),
        }
    }
}
