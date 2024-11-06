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
    let x: i32 = 1;

    let y: i32 = 13_i32;
    let f: f64 = 1.3_f64;

    let implicit_x = 1;
    let implicit_f = 1.3;

    let sum = x + y + 13;

    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    let x: &str = "hello world!";

    println!("{} {}", f, x);

    let s: String = "hello world".to_string();

    let s_slice: &str = &s;

    println!("{} {}", s, s_slice);

    let four_ints: [i32; 4] = [1, 2, 3, 4];

    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    let slice: &[i32] = &vector;

    println!("{:?} {:?}", vector, slice);

    let x: (i32, &str, f64) = (1, "hello", 3.4);

    let (a, b, c) = x;
    // let (left, right) = b.split_at(b.len()/2);
    println!("{} {} {}", a, b, c);

    println!("{}", x.1);

    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };

    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    struct TimeTravelGigawatts(f32);
    let delorean = TimeTravelGigawatts(1.21);

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

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
        fn bar_mut(&mut self) -> &mut T {
            &mut self.bar
        }
        fn into_bar(self) -> T {
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.bar());

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

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    type FunctionPointer = fn(u32) -> u32;

    let fib: FunctionPointer = fibonacci;
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

    let mut mine: Box<i32> = Box::new(3);
    *mine = 5;

    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("{}", now_its_mine);
    // println!("{}", mine); // this would not compile because `now_its_mine` now owns the pointer


    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var);
    println!("{}", *ref_var);

    ref_var;
    var = 2;

    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2;

    println!("{}", *ref_var2);

    ref_var2;
}
