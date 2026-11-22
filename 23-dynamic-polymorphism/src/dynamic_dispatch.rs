use crate::drawable::Drawable;

#[derive(Default)]
pub struct DynCanvas {
    shapes: Vec<Box<dyn Drawable>>,
}

impl DynCanvas {
    pub fn draw_shape<T: Drawable + ?Sized>(shape: &T) -> String {
        format!("Drawing {}\n{}", shape.shape_name(), shape.draw())
    }

    pub fn add<S: Drawable + 'static>(&mut self, shape: S) {
        self.shapes.push(Box::new(shape));
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
            .map(|shape| Self::draw_shape(shape.as_ref()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn draw(&self) {
        println!("{}", self.render());
    }
}

pub fn describe_borrowed(shape: &dyn Drawable) -> String {
    format!("{}: {}", shape.shape_name(), shape.draw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawable::{Circle, Rect};

    #[test]
    fn canvas_stores_different_concrete_types() {
        let mut canvas = DynCanvas::default();
        assert!(canvas.is_empty());

        canvas.add(Rect::new(0, 0, 20, 10));
        canvas.add(Circle::new(15, 5, 5));

        assert_eq!(canvas.len(), 2);
        let rendered = canvas.render();
        assert!(rendered.starts_with("Drawing Rectangle\n  move to (0, 0)"));
        assert!(rendered.contains("Drawing Circle\n  move to (15, 5)"));
    }

    #[test]
    fn borrowed_trait_object_dispatches_to_concrete_impl() {
        let circle = Circle::new(1, 2, 3);

        assert_eq!(
            describe_borrowed(&circle),
            "Circle:   move to (1, 2)\n  encircle with 3px"
        );
    }
}
