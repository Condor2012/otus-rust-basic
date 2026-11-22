use crate::drawable::{Circle, Drawable, Rect};

#[derive(Debug)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
}

impl From<Rect> for Shape {
    fn from(value: Rect) -> Self {
        Self::Rect(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}

impl Drawable for Shape {
    fn shape_name(&self) -> &'static str {
        match self {
            Self::Rect(rect) => rect.shape_name(),
            Self::Circle(circle) => circle.shape_name(),
        }
    }

    fn draw(&self) -> String {
        match self {
            Self::Rect(rect) => rect.draw(),
            Self::Circle(circle) => circle.draw(),
        }
    }
}

#[derive(Default)]
pub struct EnumCanvas {
    shapes: Vec<Shape>,
}

impl EnumCanvas {
    pub fn draw_shape<T: Drawable>(shape: &T) -> String {
        format!("Drawing {}\n{}", shape.shape_name(), shape.draw())
    }

    pub fn add<S: Into<Shape>>(&mut self, shape: S) {
        self.shapes.push(shape.into());
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn render(&self) -> String {
        self.shapes
            .iter()
            .map(Self::draw_shape)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn draw(&self) {
        println!("{}", self.render());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_variants_forward_drawable_methods() {
        let rect = Shape::from(Rect::new(0, 0, 20, 10));
        let circle = Shape::from(Circle::new(15, 5, 5));

        assert_eq!(rect.shape_name(), "Rectangle");
        assert_eq!(circle.shape_name(), "Circle");
        assert!(circle.draw().contains("encircle with 5px"));
    }

    #[test]
    fn canvas_accepts_values_through_into() {
        let mut canvas = EnumCanvas::default();
        canvas.add(Rect::new(0, 0, 20, 10));
        canvas.add(Circle::new(15, 5, 5));

        assert_eq!(canvas.len(), 2);
        assert!(!canvas.is_empty());
        assert!(canvas.render().contains("Drawing Rectangle"));
        assert!(canvas.render().contains("Drawing Circle"));
    }
}
