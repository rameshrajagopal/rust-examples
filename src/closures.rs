fn main() {
    let closure_annotated = |var: i32| -> i32 { var+1 };
    let increment    = |num| num + 1;
    let no_arg_closure = || 1;

    println!("{} {} {}", closure_annotated(10), increment(100), no_arg_closure());
}
