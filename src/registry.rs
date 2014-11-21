use std::collections::HashMap;

use metric::Metric;

pub trait Registry<T: Metric> {
    fn get<T>(&self, name: &str) -> Option<&T>;

    fn insert<T>(&mut self, name: &str, metric: T);
}

pub struct StdRegistry<T: Metric> {
    metrics: HashMap<String, T>,
}

// Specific stuff for registry goes here
impl<T: Metric> Registry<T> for StdRegistry<T> {
    fn get<T>(&self, name: &str) -> Option<&T> {
        self.metrics.get(&name.to_string())
    }

    fn insert<T>(&mut self, name: &str, metric: T) {
        self.metrics.insert(name.to_string(), metric);
    }
}

// General StdRegistry
impl<T: Metric> StdRegistry<T> {
    fn new() -> StdRegistry<T> {
        StdRegistry{
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

        r.insert("foo", m);
        r.get("foo");
    }
}
