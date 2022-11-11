use std::rc::Rc;
use std::time::SystemTime;
use crate::curve::{CURVE_ORDER, Point};
use crate::dlog::DLogProof;
use crate::utils::generate_rand_num;

mod curve;
mod utils;
mod dlog;

fn main() {
    let sid = "sid";
    let pid = 1;
    let base_point: Rc<Box<Point>>  = Rc::new(Box::new(Point::from(0)));

    let x = generate_rand_num();
    println!("{x}");
    let y = x * CURVE_ORDER;
    let y_point: Rc<Box<Point>> = Rc::new(Box::new(y.into()));

    let start_proof = SystemTime::now();
    let mut dlog_proof = DLogProof::prove(sid, pid, x, y_point.clone(), base_point.clone());

    println!("proof computation time: {} ms\n", start_proof.elapsed().unwrap().as_millis());

    println!("{} {}", dlog_proof.t.x, dlog_proof.t.y);
    println!("{}", dlog_proof.s);

    let start_verify = SystemTime::now();
    let result = dlog_proof.verify(sid, pid, y_point, base_point);

    println!("Verify computation time: {} ms", start_verify.elapsed().unwrap().as_millis());

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
