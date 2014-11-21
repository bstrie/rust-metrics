use std::collections::HashMap;

use metric::Metric;

pub trait Registry<'a> {
    fn get(&self, name: &str) -> Option<&Box<Metric+'a>>;

    fn insert(&mut self, name: &str, metric: Box<Metric+'a>);
}

pub struct StdRegistry<'a> {
    metrics: HashMap<String, Box<Metric+'a>>,
}

// Specific stuff for registry goes here
impl<'a> Registry<'a> for StdRegistry<'a> {
    fn get(&self, name: &str) -> Option<&Box<Metric+'a>> {
        self.metrics.get(&name.to_string())
    }

    fn insert(&mut self, name: &str, metric: Box<Metric+'a>) {
        self.metrics.insert(name.to_string(), metric);
    }
}

// General StdRegistry
impl<'a> StdRegistry<'a> {
    fn new() -> StdRegistry<'a> {
        StdRegistry {
            metrics: HashMap::new()
        }
    }
}

#[cfg(test)]
mod test {
    use metric::Metric;
    use meter::StdMeter;
    use registry::{Registry, StdRegistry};

    #[test]
    fn meter() {
        let mut r: StdRegistry = StdRegistry::new();
        let mut m: StdMeter = StdMeter::new();

        r.insert("foo", box m);
        r.get("foo");
    }
}
