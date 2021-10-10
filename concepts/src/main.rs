use std::num::Wrapping;
fn main() {
    let x = 1;
    println!("x is {}", x);
    // Shadowing old variable 'x', this is not the variable bind with value 1
    let x = 2;
    println!("x is {}", x);
    {
        let x = x * 2;
        println!("x in inner scope is {}", x);
    }
    println!("x in outer scope is still {}", x);

    const SECOND_PER_HOUR: i32 = 60 * 60;
    println!("每经过1小时，就有{}秒过去", SECOND_PER_HOUR);

    let y = String::from("hello, world");
    println!("y is {}", y);

    let y = y.len(); // As shadowing is creating new variable use the same name, it's okay to assign a different type
    println!("y is {}", y);

    let mut z = Wrapping(0xf0u8); // Arithmetic overflow in debug mode cause panic, and wrap in release mode.
    for _i in 1..=0x11 {
        z += Wrapping(1);
    }
    println!("z is {}", z.0); // u8 overflow here. Use Wrapping to avoid panic. eq to 1
}
