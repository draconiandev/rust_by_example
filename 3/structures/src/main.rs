/*
* There are three types of structures ("structs") that can be created using the struct keyword:
*
* Tuple structs, which are, basically, named tuples.
* The classic C structs
* Unit structs, which are field-less, are useful for generics.
*/

// A unit struct
#[derive(Debug)]
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    (rectangle.p2.x - rectangle.p1.x) * (rectangle.p2.y - rectangle.p1.y)
}

fn square(point: Point, length :f32) -> Rectangle {
    Rectangle { p1: point, p2: Point { x: length, y: length } }
}

fn main() {
    // Instantiate diagonally opposite points
    let point:        Point = Point { x: 0.0, y: 0.0 };
    let second_point: Point = Point { x: 0.5, y: 0.6 };

    let square_point: Point = Point { x: 0.1, y: 0.1 };

    // Access the fields of the point
    println!("first point coordinates:  ({}, {})", point.x, point.y);
    println!("second point coordinates: ({}, {})", second_point.x, second_point.y);

    // Destructure the point using a `let` binding
    let rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: point,
        p2: second_point,
    };

    let square = square(square_point, 0.3);

    println!("Rectangle: {:?}", rectangle);
    println!("Rectabgle Area: {:.2} square units", rect_area(rectangle));
    println!("Square: {:?}", square);
    println!("Square Area: {:.2} square units", rect_area(square));

    // Instantiate a unit struct
    let nil = Nil;
    println!("Nil: {:?}", nil);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}