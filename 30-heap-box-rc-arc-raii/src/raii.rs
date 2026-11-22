use std::cell::Cell;
use std::rc::Rc;

pub struct Resource {
    name: &'static str,
    drops: Rc<Cell<usize>>,
}

impl Resource {
    pub fn new(name: &'static str, drops: &Rc<Cell<usize>>) -> Self {
        Self {
            name,
            drops: Rc::clone(drops),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

pub fn drops_after_scope() -> usize {
    let drops = Rc::new(Cell::new(0));

    {
        let _first = Resource::new("память", &drops);
        let _second = Resource::new("файл", &drops);
        assert_eq!(drops.get(), 0);
    }

    drops.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_runs_automatically_for_every_owner() {
        assert_eq!(drops_after_scope(), 2);
    }

    #[test]
    fn explicit_drop_releases_resource_early() {
        let drops = Rc::new(Cell::new(0));
        let resource = Resource::new("сокет", &drops);

        assert_eq!(resource.name(), "сокет");
        drop(resource);
        assert_eq!(drops.get(), 1);
    }
}
