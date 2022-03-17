struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Iterate over items in a vector by value.
    let points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let _first_point: Point = points.into_iter().next().unwrap();

    //iterate over items in a vector by reference
    let points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let mut iter = points.iter();
    // points.iter() is equivalent to (&points).into_iter()
    let _first_point: &Point = iter.next().unwrap();

    // Iterate over items in a vector by mutable reference:
    let mut points = vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }];
    let mut iter = points.iter_mut();
    // points.iter_mut() is equivalent to (&mut points).into_iter()

    let first_point: &mut Point = iter.next().unwrap();
    first_point.x = 3.0;
    first_point.y = 4.0;
}
