use std::{f32::consts::PI, thread::current};

use geo_types::Point;
use rug::ops::Pow;

#[derive(Debug)]
struct LA {
    length: f64,
    angle: f64
}

pub const SMOOTHING: f64 = 0.001; 

fn line(point_a: Point<f64>,  point_b: Point<f64>) -> LA {
    //
    let l_x = point_b.x() - point_a.x();
    let l_y = point_b.y() - point_a.y();

    LA {
        length: l_x.pow(2).sqrt() + l_y.pow(2),
        angle: l_y.atan2(l_x)
    }
}

fn control_point(current: Point<f64>, previous: Option<Point<f64>>, next: Option<Point<f64>>, reverse: Option<bool>) -> Point<f64> {
    //
    let p = match previous {
        Some(p) => p,
        None => current 
    };

    let n = match next {
        Some(p) => p,
        None => current 
    };
    
    let o = line(p, n);
    
    let angle = o.angle + (if reverse.unwrap_or(false) { PI as f64} else { 0.0 });
    let length = o.length * SMOOTHING;
        
    (current.x() + angle.cos() * length,
        current.y() + angle.sin() * length
    ).into()
}

fn bezier_command(point: Point<f64>, i: i16, a: Vec<Point<f64>>) -> String {
    //
    use std::convert::TryFrom;

    let index = usize::try_from(i).unwrap();
    
    let prev = a.get(index-1).cloned();
    let prev_of_prev = if index > 1 { a.get(index-2).cloned() } else { None };
    let next = a.get(index+1).cloned();

    let cps = control_point(prev.unwrap(), prev_of_prev, Some(point), None);
    let cpe = control_point(point, prev, next, Some(true));
    
    format!("C {},{} {},{} {},{}", cps.x().round() as i64, cps.y().round() as i64, cpe.x().round() as i64, cpe.y().round() as i64, point.x().round() as i64, point.y().round() as i64)
}

pub fn svg_path(points: Vec<Point<f64>>) -> String {
    let d = points.iter().enumerate().fold("".to_string(), |acc, (i, point)| {
        if i == 0 { format!("{}M {},{}", acc, point.x(), point.y()) }
        else {
            format!("{} {}", acc, bezier_command(point.clone(), i as i16, points.clone()))
        }
    });
    format!("{}", d)
}

#[test]
fn test() {

    let points: Vec<Point<f64>> = vec!(
        (5527.0, 4565.0).into(),
        (5519.0, 4570.0).into(),
        (5505.0, 4580.0).into(),
        (5492.0, 4597.0).into(),
        (5481.0, 4614.0).into(),
        (5471.0, 4634.0).into(),
        (5463.0, 4655.0).into(),
        (5462.0, 4677.0).into(),
        (5464.0, 4698.0).into(),
        (5469.0, 4713.0).into(),
        (5476.0, 4726.0).into(),
        (5499.0, 4746.0).into(),
        (5561.0, 4774.0).into(),
        (5990.0, 4948.0).into()
    );

    // println!("{:?}", line(points[0], points[1]));
    
    // println!("{:?}", control_point(points[1], points[0], points[2] Some(true)));

    // println!("{:?}", bezier_command(points[2], 2, points));
    
    // println!("{:?}", svg_path(points, bezier_command));
}