use sdl2::rect::{Point, FPoint};
use thebox::{angle_point, angle_fpoint, geometry};

#[test]
fn check_angle(){
    let point1 = angle_point((0, 0), 90.0, 1.0);
    let point2 = angle_point((0, 0), 180.0, 1.0);
    assert_eq!(point1, Point::new(1, 0));
    assert_eq!(point2, Point::new(0, 1));
}

#[test]
fn check_angle_float(){
    let point1 = angle_fpoint((0.0, 0.0), 90.0, 1.0);
    let point2 = angle_fpoint((0.0, 0.0), 180.0, 1.0);
    assert_eq!(point1, FPoint::new(1.0, 0.00000004371139)); //Thanks float precision!
    assert_eq!(point2, FPoint::new(-0.00000008742278, 1.0));
}

#[test]
fn check_geometry(){
    let points: Vec<Point> = geometry((0, 0), 2, 1.0);
    assert_eq!(points, vec!(Point::new(0, -1), Point::new(0, 1)));
}