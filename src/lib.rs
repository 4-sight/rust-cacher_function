use std::collections::HashMap;

pub struct Cache<T>
where
    T: Fn(u8) -> u32,
{
    calc: T,
    values: HashMap<u8, u32>,
}

impl<T> Cache<T>
where
    T: Fn(u8) -> u32,
{
    pub fn new(calc: T) -> Cache<T> {
        Cache {
            calc,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u8) -> u32 {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calc)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

// ======================================================
// ======================================================

#[cfg(test)]

mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use std::time::Instant;

    fn get_expensive_function(delay: u64) -> impl Fn(u8) -> u32 {
        move |num: u8| {
            thread::sleep(Duration::from_millis(delay));
            num as u32 * 100
        }
    }

    #[test]
    fn correct_value() {
        let mut cache = Cache::new(get_expensive_function(0));

        assert_eq!(100, cache.value(1));
        assert_eq!(200, cache.value(2));
    }

    #[test]
    fn caches() {
        let mut cache = Cache::new(get_expensive_function(200));

        let start_time = Instant::now();
        cache.value(1);
        assert!(start_time.elapsed().as_millis() == 200);
        cache.value(1);
        assert!(start_time.elapsed().as_millis() == 200);
        cache.value(2);
        assert!(start_time.elapsed().as_millis() == 400);
        cache.value(2);
        assert!(start_time.elapsed().as_millis() == 400);
    }
}
