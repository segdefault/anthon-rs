use std::collections::LinkedList;
use std::ops;
use std::ops::Deref;

use num::{Float, NumCast};

#[allow(dead_code)]
pub type Wmaf32 = WeightedMovingAverage<f32>;
#[allow(dead_code)]
pub type Wmaf64 = WeightedMovingAverage<f64>;

pub struct WeightedMovingAverage<T: Float> {
    values: LinkedList<T>,
    value: T,
    order: usize,
}

impl<T: Float> WeightedMovingAverage<T> {
    pub fn new(order: usize) -> Self {
        Self {
            values: LinkedList::new(),
            value: Float::neg_zero(),
            order,
        }
    }

    pub fn new_from(order: usize, value: T) -> Self {
        let mut weighted_float: WeightedMovingAverage<T> = WeightedMovingAverage::new(order);
        weighted_float.set_value(value);

        weighted_float
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        if self.values.len() == self.order {
            self.values.pop_front();
        }

        self.values.push_back(value);
        self.update_value();
    }

    fn update_value(&mut self) {
        self.value = match self.values.len() {
            0 => Float::neg_zero(),
            1 => *self.values.front().unwrap(),
            _ => {
                let sum = self
                    .values
                    .iter()
                    .enumerate()
                    .fold(Float::neg_zero(), |acc: T, (n, v)| {
                        acc + *v * NumCast::from(self.values.len() - n).unwrap()
                    });

                let a: T = NumCast::from(self.values.len() * (self.values.len() + 1)).unwrap();
                let b: T = NumCast::from(2f32).unwrap();
                let denominator = a / b;

                sum / denominator
            }
        }
    }
}

impl<T: Float> Deref for WeightedMovingAverage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T: Float> ops::Add<T> for WeightedMovingAverage<T> {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self += self.value + rhs;

        self
    }
}

impl<T: Float> ops::AddAssign<T> for WeightedMovingAverage<T> {
    fn add_assign(&mut self, rhs: T) {
        self.set_value(self.value + rhs)
    }
}

impl<T: Float> ops::Sub<T> for WeightedMovingAverage<T> {
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        self += self.value - rhs;

        self
    }
}

impl<T: Float> ops::SubAssign<T> for WeightedMovingAverage<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.set_value(self.value - rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::common::filter::Wmaf32;

    #[test]
    fn unset() {
        let v = Wmaf32::new(10);

        assert_eq!(*v, -0f32);
    }

    #[test]
    fn one_value() {
        let mut v = Wmaf32::new(10);
        v.set_value(10f32);

        assert_eq!(*v, 10f32);
    }

    #[test]
    fn two_equal_values() {
        let mut v = Wmaf32::new(10);

        let real = 10f32;
        v.set_value(real);
        v.set_value(real);

        assert_eq!(*v, real);
    }

    #[test]
    fn three_equal_values() {
        let mut v = Wmaf32::new(10);

        let real = 10f32;
        v.set_value(real);
        v.set_value(real);
        v.set_value(real);

        assert_eq!(*v, real);
    }

    #[test]
    fn drop_old_value() {
        let mut v = Wmaf32::new(3);

        let real = 10f32;
        v.set_value(real * 2f32);
        v.set_value(real);
        v.set_value(real);
        v.set_value(real);

        assert_eq!(*v, real);
    }
}
