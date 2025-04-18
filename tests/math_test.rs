use sdl3::rect::Point;
use sdl3::render::FPoint;
use thebox::{
    angle_point,
    angle_fpoint,
    angle_difference_cos,
    angler_difference_cos,
    angle_difference_sin,
    angler_difference_sin,
    geometry
};

#[test]
fn check_angle(){
    let point1 = angle_point((0, 0), 90_f32, 1_f32);
    let point2 = angle_point((0, 0), 180_f32, 1_f32);
    assert_eq!(point1, Point::new(1, 0));
    assert_eq!(point2, Point::new(0, 1));
}

#[test]
fn check_angle_float(){
    let point1 = angle_fpoint((0_f32, 0_f32), 90_f32, 1_f32);
    let point2 = angle_fpoint((0_f32, 0_f32), 180_f32, 1_f32);
    assert_eq!(point1, FPoint::new(1_f32, 0.00000004371139)); //Thanks float precision!
    assert_eq!(point2, FPoint::new(-0.00000008742278, 1_f32));
}

#[test]
fn check_geometry(){
    let points: Vec<FPoint> = geometry((0, 0), 2, 1_f32);
    assert_eq!(points, vec!(FPoint::new(0., -1.), FPoint::new(0., 1.)));
}

#[test]
fn check_angle_differences(){
    assert_eq!(angle_difference_cos(0_f32, 180_f32), -1.0);
    assert_eq!(angler_difference_cos(0_f32.to_radians(), 0_f32.to_radians()), 1.0);
}

#[test]
fn check_angle_radians_differences() {
    assert_eq!(angle_difference_sin(0_f32, 90_f32), 1.0);
    assert_eq!(angler_difference_sin(0_f32.to_radians(), -90_f32.to_radians()), -1.0);
}