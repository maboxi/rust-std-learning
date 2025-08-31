# Rust Standard Libary Learning

I am using this repository to study the rust standard library, either by creating small examples / POCs or by recreating functionalities myself.

## Pointers and interior mutability

Interior mutability: change contained value while only having shared access to the container

### `std::cell::Cell`

Description: [Cell in std::cell](doc.rust-lang.org/std/cell/struct.Cell.html)

Provides interior mutability:
- read value by copying it (-> no references given out)
- change value by overwriting it

### `std::cell::RefCell`

Description: [RefCell in std::cell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

Interior mutability possible due to runtime reference counting:
- if there is **no** exclusive ref given out a.t.m., can give out arbitrary number of shared references through `Ref` object, which will update refcount on drop
- if there are **no** exclusive and **no** shared references given out a.t.m., can give out a single exclusive reference allowing mutable access through a `RefMut` object, which will update the refcount on drop

### `std::rc::Rc`

Description: [Rc in std::rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)

= smart pointer (C++ std::shared)
- count references
- drop object if last reference was dropped
- only gives shared access to contained object -> combine with interior mutability container
- `Rc: !Sync` and `Rc: !Send` -> not thread-safe -> see std::sync::Arc

### `std::sync::Arc`

Description: [Arc in std::sync](https://doc.rust-lang.org/std/sync/struct.Arc.html)

= thread-safe smart pointer 
- similar to `std::rc::Rc`
- uses atomics for reference counting -> thread-safe

## Collections

### `std::vec::Vec`

Description: [Vec in std::vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

### `std::collections::VecDeque`

Description: [VecDeque in std::collections](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

### `std::collections::LinkedList`

Description: [LinkedList in std::collections](https://doc.rust-lang.org/std/collections/linked_list/index.html)

### `std::collections::HashMap`

Description: [HashMap in std::collections](https://doc.rust-lang.org/std/collections/struct.HashMap.html)