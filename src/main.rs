use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

// inspired by these:
//  - https://rust-unofficial.github.io/too-many-lists/second-final.html
//  - https://www.youtube.com/watch?v=IiDHTIsmUi4 

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    
    // this inserts at the head (start)
    fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Rc::new(RefCell::new(
            Node {
                element,
                next:old_head,
            }
        ));
        self.head = Some(new_head);
    }

    // this pops from the head (start)
    // largely inspired by https://rust-unofficial.github.io/too-many-lists/fourth-final.html but only a singly linked list
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.borrow_mut().next.take();
            let x3 = Rc::try_unwrap(old_head).ok(); // x3 is now an Option<RefCell<Node<T>>>. This is how you get the thing out of an Rc
            x3.unwrap().into_inner().element
        })
    }

    fn peek(&self) -> Option<Ref<T>> {
        let x = self.head.as_ref();
        x.map(|node| {
            Ref::map(node.borrow(), |n| &n.element)
        })
    }

    fn peek_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |n| &mut n.element)
        })
    }
}

fn main() {
    let exp_size: u32 = 10;

    let mut list = LinkedList::<u32>::new();
    
    for i in 0..10_000 {
        if i % 10 == 0 {
          println!("{}", i);
        }
        list.push(i);
    }

    let popped = list.pop();
    println!("popped element: {:?}", popped);

    let popped_2 = list.pop();
    println!("popped_2 element: {:?}", popped_2);

    let p = list.peek();
    if let Some(p_some) = p {
        println!("p_some: {}", p_some);
    }

    let mut p_mut = list.peek_mut();
    if let Some(mut p_some_mut) = p_mut {
        println!("p_some_mut: {}", p_some_mut);
        *p_some_mut = 42;
        println!("p_some_mut: {}", p_some_mut);
    }

    println!("done");
    // println!("list: {:?}", list);
}
