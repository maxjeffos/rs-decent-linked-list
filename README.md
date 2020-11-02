# rs-decent-linked-list

A descent linked list in Rust using `Rc<RefCell<Node<T>>>`.

The `RefCell<T>` allows for "interior mutability", meaning you can mutate the thing inside, i.e. the `T`.

The `Rc<T>` allows for shared ownership (caveat: it will blow up at runtime if you break the Rust borrowing rules). Probably not strictly required for a singly linked list such as this, but it is a sufficient launch pad into a more advanced structure like a doubly linked list, a tree, or a graph, where you'd need something like this.

Inspired by these:
- https://rust-unofficial.github.io/too-many-lists/second-final.html
- https://www.youtube.com/watch?v=IiDHTIsmUi4 
