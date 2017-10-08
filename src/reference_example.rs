
fn reference_func() {
    let reference = &10;

    println!("ref is {:?}", reference);
    match reference {
        &val => println!("ref is {:?}", val),
    }

    let value = 10;
//    let mut mutable_value = 200;

    let mut pair = (value, 200);
    match pair {
        (10, ref mut m) => {
            *m += 10;
            println!("m {}", m);
        },
        (_, ref mut m) => {
            *m += 200;
            println!("m {}", m);
        }
    }
}

fn age() -> u32 {
    13
}

fn match_with_binding() {
    match age() {
        0 => println!("zero"),
        n @ 1 ... 12 => println!("age of {}", n),
        n @ 13 ... 19 => println!("teen age {}", n),
        _ => println!("beyond 20"),
    }
}

fn struct_func() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo { x: (1, 2), y: 10 };
    let Foo {x: (a, b), y } = foo;

    println!("a={}, b={}, y={}", a, b, y);

    // struct de structuring 
    let Foo {y: i, x : j} = foo;
    println!("i = {:?}, j = {:?}", i, j);
}

fn main() {
    reference_func();
    struct_func();
    match_with_binding();
}
