#![no_std]

use num_integer::Integer;
use num_rational::Ratio;
use num_traits::One;
pub fn stern<I: Integer + Clone, J: Integer>(i: I) -> J {
    if i == I::one() {
        return J::one();
    }
    let i2 = i.clone() / (I::one() + I::one());
    let j2 = stern(i2.clone());
    let j2 = if i2.clone() * (I::one() + I::one()) == i {
        j2
    } else {
        j2 + stern(i2 + I::one())
    };
    return j2;
}
pub fn stern_ratio<I: Integer + Clone, J: Integer>(i: I) -> Ratio<J> {
    Ratio::new_raw(stern(i.clone()), stern(i + I::one()))
}
pub fn destern_ratio<I: Integer + Clone, J: Integer + Clone>(i: Ratio<I>) -> J {
    match i.numer().cmp(i.denom()) {
        core::cmp::Ordering::Less => match i.clone() / (Ratio::<I>::one() - i) {
            x => match destern_ratio::<I, J>(x) {
                v => v.clone() + v,
            },
        },
        core::cmp::Ordering::Equal => J::one(),
        core::cmp::Ordering::Greater => match destern_ratio::<I, J>(i - Ratio::<I>::one()) {
            v => v.clone() + v + J::one(),
        },
    }
}
