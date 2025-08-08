use std::fmt;

// A concrete type `A`.
struct A;

// Implement Debug for A
impl fmt::Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A")
    }
}

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);

// Implement Debug for Single
impl fmt::Debug for Single {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Single({:?})", self.0)
    }
}

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
struct SingleGen<T>(T);

// Implement Debug for SingleGen<T> where T: Debug
impl<T: fmt::Debug> fmt::Debug for SingleGen<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SingleGen({:?})", self.0)
    }
}

fn main() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);
    println!("{:?}", _s);
    
    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`.
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');
    println!("{:?}", _char);

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.

    println!("{:?}", _t);
    println!("{:?}", _i32);
    println!("{:?}", _char);
}