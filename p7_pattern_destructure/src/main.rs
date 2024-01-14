fn test1() {
    let tuple = (1i32, false, 3f32);
    let (head, center, tail) = tuple;
    println!("head: {} center: {} tail: {}", head, center, tail);
}

enum Direction {
    East,
    West,
    South,
    North,
}
fn print_direction(x: Direction) {
    match x {
        Direction::East => {
            println!("East!!");
        }
        Direction::West => {
            println!("West!!");
        }
        Direction::South => {
            println!("South!!");
        }
        _ => {
            println!("Other!!")
        }
    }
}
fn test2() {
    let x = Direction::North;
    print_direction(x);
}

fn test3() {
    fn direction_to_int(x: Direction) -> i32 {
        match x {
            Direction::East => 0,
            Direction::West => 1,
            Direction::South => 2i32,
            Direction::North => 3,
        }
    }
    let x = Direction::North;
    let s = direction_to_int(x);
    println!("s: {}", s);
}

fn test4() {
    enum  OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(6);
    let y: OptionalInt = OptionalInt::Missing;
    match x {
        OptionalInt::Value(i) if i >5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("x is an integer!"),
        OptionalInt::Missing => println!("x is missing"),
    }

}

fn test5() {
    struct T {
        item1: char,
        item2: bool,
    }
    fn test( T{item1: arg1, item2: arg2} : T) {
        println! ("{} {}", arg1, arg2);
    }
    
    let x = T {
        item1: 'A',
        item2: false,
    };
    test(x);
    
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}
