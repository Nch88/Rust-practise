use turtle::Turtle;

pub fn square(turtle: &mut Turtle) {
    for _ in 0..4 {
        turtle.forward(250.0);
        turtle.right(90.0);
    }
    turtle.clear();
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn shift_origin(turtle: &mut Turtle, dir: Direction) {
    turtle.pen_up();
    match dir {
        Direction::Left => {
            turtle.left(90.0);
            turtle.forward(300.0);
            turtle.right(90.0);
        }
        _ => unimplemented!(),
    }
    turtle.pen_down();
}

pub fn circle(turtle: &mut Turtle) {
    for _ in 0..360 {
        // Move forward three steps
        turtle.forward(5.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.0);
    }
}

fn main() {
    let mut turtle = Turtle::new();

    circle(&mut turtle);
    turtle.clear();
    shift_origin(&mut turtle, Direction::Left);
    circle(&mut turtle);
}
