use std::{collections::HashMap, thread, time::Duration};

fn main() {
    generate_worker(1, 4);

    move_test();
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_worker(intensity: u32, random_number: u32) {
    // 閉包
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculate slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        return;
    }

    if random_number != 3 {
        println!(
            "Today, run for {} minutes",
            expensive_closure.value(intensity)
        );
        return;
    }

    println!("Done");
}

fn move_test() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;
    // println!("x: {:?}", x);
    //                     ↑
    // borrow of moved value: `x`

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

#[test]
fn iterator() {
    // iterator entry
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {:?}", val);
        }
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum();
    assert_eq!(sum, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(Clone, PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_my_size(shoe: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoe.into_iter().filter(|x| x.size == size).collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("type 1"),
        },
        Shoe {
            size: 13,
            style: String::from("type 2"),
        },
        Shoe {
            size: 10,
            style: String::from("type 3"),
        },
    ];

    let result = shoes_in_my_size(shoes, 10);
    println!("{:?}", result);
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_test() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn counter_other_trait_test() {
    // 這邊要寫上 u32 => bound type
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum)
}
