use std::{f64::consts::PI};
use geo_types::Point;

#[derive(Debug)]
struct LA {
    length: f64,
    angle: f64
}

fn line(point_a: Point<f64>,  point_b: Point<f64>) -> LA {
    //
    let l_x = point_b.x() - point_a.x();
    let l_y = (-point_b.y()) - (-point_a.y());

    let l = (l_x.powf(2.) + l_y.powf(2.)).sqrt();
    let a = l_y.atan2(l_x);
    
    // println!("line: l: {} a: {}", l, a);

    LA {
        length: l,
        angle: a
    }
}

pub fn path_has_min_angle(points: Vec<Point<f64>>, min_angle_in_deg: i16, debug: bool) -> (String, bool) {
    //
    let mut has_small_angle = false;
    let d = points.iter().enumerate().fold("".to_string(), |acc, (i, point)| -> String {
        if i == 0 { format!("{}M {},{}", acc, point.x(), point.y()) 
        // } else if i == 1 {
        } else if i < points.len() - 1{
            // Check current and previous line for the angle
            let previous_line = line(points[i-1], points[i]);
            let current_line = line(points[i], points[i+1]);
            
            let angle_radian_measure = angle(previous_line, current_line);
            
            if debug { println!("angle: {}, threshold: {}", angle_radian_measure, min_angle_in_deg); }

            // Check if angle is above threshold, if yes make curve
            if angle_radian_measure < min_angle_in_deg as f64 {
                has_small_angle = true;
                if debug { println!("--> angle too small"); }
                acc
            } else {
                format!("{}L {},{}", acc, point.x(), point.y())
            }
        } else {
            format!("{}L {},{}", acc, point.x(), point.y())
        }
    });

    (format!("{}", d), has_small_angle)
}

fn angle(l1: LA, l2: LA) -> f64 {
    let pi = PI as f64;
    let unit = 180.0 / pi;

    if (l1.angle >= 0. && l2.angle >= 0.) || (l1.angle < 0. && l2.angle < 0.) {
        if l1.angle >= l2.angle {
            let l1_a_deg = 180. - (l1.angle * unit);
            let l2_a_deg = l2.angle * unit;
            l1_a_deg + l2_a_deg
        } else {
            let l1_a_deg = l1.angle * unit;
            let l2_a_deg = 180. - (l2.angle * unit);
            l1_a_deg + l2_a_deg
        }
    } else {
        let l1_a_deg = l1.angle.abs() * unit;
        let l2_a_deg = l2.angle.abs() * unit;
        180. - (l1_a_deg + l2_a_deg)
    }
}

#[test]
fn test_angles() {
    
    let both_pos = (
        angle(
        line((0.0, 0.0).into(),(3.0, 1.0).into()),
        line((3.0, 1.0).into(),(4.0, 4.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(3.0, 1.0).into()),
        line((3.0, 1.0).into(),(2.0, 4.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(3.0, 0.0).into()),
        line((3.0, 0.0).into(),(3.0, 3.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(-1.0, 3.0).into()),
        line((-1.0, 3.0).into(),(0.0, 5.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(-1.0, 3.0).into()),
        line((-1.0, 3.0).into(),(-4.0, 4.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(2.0, 1.0).into()),
        line((2.0, 1.0).into(),(0.0, 2.0).into())
        ).round(),
        angle(
        line((0.0, 0.0).into(),(3.0, 0.0).into()),
        line((3.0, 0.0).into(),(0.0, 3.0).into())
        )
    );

    assert_eq!(both_pos, (127.0, 90.0, 90.0, 135.0, 127.0, 53.0, 45.0));

    let both_neg = (
        angle(
        line((0.0, 0.0).into(),(3.0, -1.0).into()),
        line((3.0, 1.0).into(),(3.0, -4.0).into())
        ).round(),
    );

    assert_eq!(both_neg, (108.0,));
    
    let one_pos_one_neg = (
        angle(
            line((0.0, 0.0).into(),(3.0, -2.0).into()),
            line((3.0, -2.0).into(),(4.0, 0.0).into())
        ).round(),
        angle(
            line((0.0, 0.0).into(),(-1.0, 2.0).into()),
            line((-1.0, 2.0).into(),(1.0, 0.0).into())
        ).round()
    );
    
    assert_eq!(one_pos_one_neg, (83.0, 18.0));
}