fn main() {
    //foo();
    //foo1();
    foo2();
}


fn foo() {
    let x = Box::new(75);
    println!("`x` points to {}", *x);
}

fn foo1() {

    // x,*x, y, *y is immutable;
    let x = Box::new(75);
    let y = Box::new(42);
    //x = y; //error[E0384]: cannot assign twice to immutable variable `x`
    //*x = 43;//error[E0594]: cannot assign to `*x`, as `x` is not declared as mutable

    //x , *x is mutable;
    let mut x = Box::new(74);
    x = y;
    println!("`x` points to {}", *x);

    *x = 43;
    println!("`x` points to {}", *x);
}

fn foo2() {
    let x = 3;
    let mut y = Box::new(x);
    *y = 45;

    println!( "x is still {}", x);
}