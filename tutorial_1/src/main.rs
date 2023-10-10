fn main() {
    let x = 4; // implicitely defining the type of int
    println!("x is: {}", x); // this is a format string

    {
        /* This is scope. I can use a variable in here no problem, 
        but it will only be used in this scope and not be used anywhere else 
        outside of the scope. However, I can use variables OUTSIDE the scope
        for the inside, like Line 12
        */
        let y = 32;
        let x = x - 2;
        println!("x is: {}", x);
        println!("y is: {}", y);
    }

    let x = x + 5;
    println!("x is: {}", x); // this is a format string
// This works cause we are 'recreating' the variable 'reassign' (but if mutable is infront, then it will error out)
// let mut reassign = 12
// reassign = "hello" THIS WILL ERROR OUT
    let reassign = 12;
    println!("reassign: {}", reassign);
    let reassign = "hello";

    println!("reassign: {}", reassign);

    // constants

    const SECONDS_IN_MINUTE: u32 = 60; 
    println!("seconds in minutes: {}", SECONDS_IN_MINUTE);

    // data types

    let integer: i32 = 2; // this stands for a signed integer representing 32 bits (i32 is default)

    /*
    These are the different signed integer types. 
    Since these are SIGNED, we can also assign a negative number
    with these
    i8
    i16
    i32
    i64
    i128
    */

    let unsigned: u32 = 2;

    /*
    unsigned integers are the exact same as signed integers
    except they cannot be assigned a negative number
    u8 
    u16
    u32
    u64
    u128

    While u and i might be similair, the range in which 
    the numbers are associated with these two differ
    u8 is 2^8. Since it is unsigned, we cannot have a negative
    number. So, techincally, this a u8 can hold a range of 0 - 255 (0 - 2^8 - 1)
    However, with an i8, we have to identify negative numbers as well
    so, an i8 can hold integers from -128 - 127 (-2^7 - 2^7 - 1)
     */

     let floating_point: f32 = 2.4;

    /*
    Floating points hold floats.
    We can have f32 and f64 floats, 
    meaning one uses 32 bits, while the other 
    uses 64 bits, respectively
    This is also called single precision (f32)
    This is also called double precision (f64)
    */

    let false_boolean_word: bool = false;
    let true_boolean_word: bool = true;

}
