use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let n = MyStruct::new(3);
    {
        let n = MyStruct::new(4);
    }
    let nm = MyStruct::new(5);
    move_me(nm);
    println!("back to Main");

    let d = Droppobales {
        ms: MyStruct::new(6),
    };

    let mut sp = SmartPointer::<i32>::new();
    sp.set(7);
    println!("SmartPointer: {}", sp.get());

    println!("Ending Main function");
}

struct MyStruct {
    num: i32,
}

impl MyStruct {
    fn new(num: i32) -> Self {
        println!("Constructing {num}");
        Self { num }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.num)
    }
}

fn move_me(x: MyStruct) {
    println!("Moved {}", x.num)
}

struct Droppobales {
    ms: MyStruct,
}

// Custom Smart Pointer

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout,
}

impl<T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);
            Self {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }

    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory for SmartPointer");
        unsafe { dealloc(self.ptr, self.layout) }
    }
}
