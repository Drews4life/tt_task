pub struct Point {
    pub x: u128,
    pub y: u128,
}

impl Point {
    pub fn bytes(&self) -> Vec<u8> { vec![] }
}

impl Into<String> for Point {
    fn into(self) -> String {
        "serialized_point".to_string()
    }
}

impl From<u128> for Point {
    fn from(_: u128) -> Self {
        Self { x: 0 , y: 0 }
    }
}

impl From<Point> for u128 {
    fn from(_: Point) -> Self {
        0
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { x: 0 , y: 0 }
    }
}

impl Copy for Point {

}

pub static CURVE_ORDER: u128 = 1;
