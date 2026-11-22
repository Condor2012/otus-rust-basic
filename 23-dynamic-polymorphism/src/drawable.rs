pub trait Drawable {
    fn shape_name(&self) -> &'static str;
    fn draw(&self) -> String;
}

#[derive(Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Drawable for Rect {
    fn shape_name(&self) -> &'static str {
        "Rectangle"
    }

    fn draw(&self) -> String {
        format!(
            "  move to ({x}, {y})\n  line {width}px left\n  line {height}px down\n  line {width}px right\n  line {height}px up",
            x = self.x,
            y = self.y,
            width = self.width,
            height = self.height
        )
    }
}

#[derive(Debug)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Self { x, y, radius }
    }
}

impl Drawable for Circle {
    fn shape_name(&self) -> &'static str {
        "Circle"
    }

    fn draw(&self) -> String {
        format!(
            "  move to ({}, {})\n  encircle with {}px",
            self.x, self.y, self.radius
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_implements_drawable() {
        let rect = Rect::new(0, 0, 20, 10);

        assert_eq!(rect.shape_name(), "Rectangle");
        assert_eq!(
            rect.draw(),
            "  move to (0, 0)\n  line 20px left\n  line 10px down\n  line 20px right\n  line 10px up"
        );
    }

    #[test]
    fn circle_implements_drawable() {
        let circle = Circle::new(15, 5, 5);

        assert_eq!(circle.shape_name(), "Circle");
        assert_eq!(circle.draw(), "  move to (15, 5)\n  encircle with 5px");
    }
}
