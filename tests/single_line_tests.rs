use single_line_macro::single_line as sl;

/// A 2D point.
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    sl![/// Creates a new `Point`.
        pub new(x: i32, y: i32) -> Self => Self { x, y }];

    sl![/// Returns the X coordinate.
        pub fn x(&self) -> i32 => x];

    sl![/// Returns the Y coordinate.
        pub fn y(&self) -> i32 => y];

    sl![/// Computes the Manhattan distance from the origin.
        fn manhattan(&self) -> i32 => self.x.abs() + self.y.abs()];

    sl![/// Scales the point by a factor.
        pub fn scale(&self, f: i32) -> Self => Self { x: self.x * f, y: self.y * f }];

    sl![/// Resets the point to the origin.
        pub fn reset(&mut self) => { self.x = 0; self.y = 0 }];

    sl![pub async rdn() -> i32 => 20];
}

// Free functions
sl![pub origin -> Point => Point { x: 0, y: 0 }];
sl![distance(a: &Point, b: &Point) -> i32 => (a.x - b.x).abs() + (a.y - b.y).abs()];
sl![is_same(a: &Point, b: &Point) -> bool => a == b];

#[test]
fn test_point_methods() {
    let mut p = Point::new(3, -4);
    assert_eq!(p.x(), 3);
    assert_eq!(p.y(), -4);
    assert_eq!(p.manhattan(), 7);
    assert_eq!(p.scale(2), Point { x: 6, y: -8 });

    p.reset();
    assert_eq!(p, Point { x: 0, y: 0 });
}

#[test]
fn test_point_functions() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(-3, 5);
    assert_eq!(origin(), Point { x: 0, y: 0 });
    assert_eq!(
        distance(&p1, &p2),
        (1i32 - -3i32).abs() + (2i32 - 5i32).abs()
    );
    assert_eq!(is_same(&p1, &p1), true);
    assert_eq!(is_same(&p1, &p2), false);
}
