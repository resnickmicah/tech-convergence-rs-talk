#[allow(dead_code)]
fn add2(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
fn explicit_add2(x: i32, y: i32) -> i32 {
    return x + y;
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Declaration, Initialization and numeric types
    let x: i32 = 1;

    let y: i32 = 13_i32;
    let f: f64 = 1.3_f64;

    let implicit_x = 1;
    let implicit_f = 1.3;

    let sum = x + y + 13;

    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings and &strs
    let f: f64 = 1.3_f64;
    let x: &str = "hello world!";
    println!("{} {}", f, x);
    let h: String = "Hello".to_string();
    let d: String = "Dan".to_string();
    let b: String = "Beetlejuice Beetlejuice Beetlejuice".to_string();
    let b_slice: &str = &b;
    println!("{} {}", h, b_slice);

    // Arrays and vectors
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    let slice: &[i32] = &vector;

    println!("{:?} {:?}", vector, slice);

    // Tuples
    let x: (i32, &str, f64) = (1, "yes no", 1.21);

    let (the_loneliest_number, hello_goodbye, gigawatts) = x;
    let (you_say, i_say) = hello_goodbye.split_at(3);
 
    println!("{} {} {}", the_loneliest_number, hello_goodbye, gigawatts);
    println!("{} {}", i_say, you_say);
    println!("{}", x.1);

    // Structs and Tuple Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };

    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    struct TimeTravelGigawatts(f32);
    let delorean = TimeTravelGigawatts(1.21);

    // C-style enums
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // Enums with variants that contain values
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing = OptionalI32::Nothing;

    struct Foo<T> {
        bar: T,
    }

    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    impl<T> Foo<T> {
        fn bar(&self) -> &T {
            &self.bar
        }
        // need to borrow
        fn bar_mut(&mut self) -> &mut T {
            &mut self.bar
        }
        fn into_bar(self) -> T {
            self.bar
        }
    }
    // Traits
    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }
    let another_foo = Foo { bar: 1 };
    println!("{:?}", another_foo.frobnicate());
    // Also works, but not after the first frobnicate call.
    // println!("{:?}", Frobnicate::frobnicate(another_foo));

    // Match expression and type keyword
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    type FunctionPointer = fn(u32) -> u32;

    let fib: FunctionPointer = fibonacci;
    // don't need to declare `type` here, functions are first class.
    let fib: fn(u32) -> u32  = fibonacci;
    println!("Fib: {}", fib(4));

    let foo = OptionalI32::AnI32(1);
    match foo {
        OptionalI32::AnI32(n) => println!("it’s an i32: {}", n),
        OptionalI32::Nothing => println!("it’s nothing!"),
    }

    struct FooBar {
        x: i32,
        y: OptionalI32,
    }
    let bar = FooBar {
        x: 15,
        y: OptionalI32::AnI32(32),
    };

    match bar {
        FooBar {
            x: 0,
            y: OptionalI32::AnI32(0),
        } => println!("The numbers are zero!"),
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } if n == m => println!("The numbers are the same"),
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } => println!("Different numbers: {} {}", n, m),
        FooBar {
            x: _,
            y: OptionalI32::Nothing,
        } => println!("The second number is Nothing!"),
    }

    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    for i in 0u32..10 {
        print!("{} ", i);
    }
    println!("");

    if 1 == 1 {
        println!("Maths is working!");
    } else {
        println!("Oh no...");
    }

    let value = if true { "good" } else { "bad" };

    while 1 == 1 {
        println!("The universe is operating normally.");
        break;
    }

    loop {
        println!("Hello!");
        break;
    }

    // Memory safety, transferring ownership
    let mut joeys_box: Box<i32> = Box::new(3);
    *joeys_box = 5;
    let mut tommys_box_now = joeys_box;
    *tommys_box_now += 2;
 
    println!("{}", tommys_box_now);
    // the below would not compile because `tommys_box_now` now owns the box
    // println!("{}", joeys_box);

    // Immutable references and borrowing
    let mut magic_number = 4;
    magic_number = 3;
    let inspector_gadget: &i32 = &magic_number;
    
    println!("{}", magic_number);
    println!("{}", inspector_gadget);
    // magic_number = 2; // can't compile, inspector gadget has it
    inspector_gadget;
    // in the clear now, it isn't being borrow anymore
    magic_number = 2;

    // mutably borrowing
    let mut magic_number = 4;
    let doctor_robotnik: &mut i32 = &mut magic_number;
    *doctor_robotnik += 2;
    // won't compile, magic_number is being mutably borrowed here.
    // println!("{}", magic_number);
    // magic_number = 2;
    println!("{}", *doctor_robotnik);
    // In the clear, doctor robotnik's borrow is no longer active.
    magic_number = 2;
    println!("{}", magic_number);
}
