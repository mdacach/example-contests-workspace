#[derive(Debug)]
pub struct Combinatorics<const M: i64> {
    factorials: Vec<ModularInt<M>>,
    inverses: Vec<ModularInt<M>>,
}

impl<const M: i64> Combinatorics<M> {
    pub fn new_with_range(n: usize) -> Self {
        Self {
            factorials: ModularInt::factorial_table(n),
            inverses: ModularInt::inverse_table(n),
        }
    }

    pub fn combinations(&self, a: ModularInt<M>, b: ModularInt<M>) -> ModularInt<M> {
        let a: usize = a.try_into().unwrap();
        let b: usize = b.try_into().unwrap();
        let numerator = self.factorials[a];

        let denominator1 = self.factorials[a - b];
        let denominator2 = self.factorials[b];

        numerator / denominator1 / denominator2
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ModularInt<const M: i64> {
    pub value: i64,
}

impl<const M: i64> TryFrom<ModularInt<M>> for usize {
    type Error = String;

    fn try_from(value: ModularInt<M>) -> Result<Self, Self::Error> {
        usize::try_from(value.value)
            .map_err(|_| format!("modint {:?} can't be converted to usize", value))
    }
}

impl<const M: i64> ModularInt<M> {
    // Compute all multiplicative inverses from 1..=n in O(n) time.
    // TODO: understand this
    pub fn inverse_table(n: usize) -> Vec<Self> {
        let mut inverses = Vec::with_capacity(n + 1);
        inverses.extend_from_slice(&[Self::from_small(0), Self::from_small(1)]);
        let n = n as i64;
        for i in 2..=n {
            let (md, dv) = (M % i, M / i);
            inverses.push(inverses[md as usize] * Self::from_small(-dv));
        }
        inverses
    }

    pub fn factorial_table(n: usize) -> Vec<Self> {
        let mut factorials = Vec::with_capacity(n + 1);
        factorials.push(Self::from_small(1));
        for x in 1..=n {
            factorials.push(factorials[x - 1] * (x as i64).into())
        }
        factorials
    }

    pub fn from_small(number: i64) -> Self {
        debug_assert!(number < M);
        debug_assert!(number > -M);

        Self {
            value: if number < 0 { number + M } else { number },
        }
    }

    // [Binary Exponentiation](https://cp-algorithms.com/algebra/binary-exp.html)
    // O(log(exponent)) time.
    pub fn pow(self, mut exponent: u64) -> Self {
        let mut result = Self::from_small(1);
        let mut base = self;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            exponent = exponent / 2;
        }
        result
    }

    // [Modular Inverse](https://cp-algorithms.com/algebra/module-inverse.html)
    // Note that this only exists if `self` and `M` are relatively prime.
    pub fn multiplicative_inverse(self) -> Self {
        self.pow(M as u64 - 2)
    }
}

impl<const M: i64> From<i64> for ModularInt<M> {
    fn from(value: i64) -> Self {
        Self::from_small(value % M)
    }
}

impl<const M: i64> std::ops::Add for ModularInt<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_small(self.value + rhs.value - M)
    }
}

impl<const M: i64> std::ops::Sub for ModularInt<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_small(self.value - rhs.value)
    }
}

impl<const M: i64> std::ops::Mul for ModularInt<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.value * rhs.value)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const M: i64> std::ops::Div for ModularInt<M> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.multiplicative_inverse()
    }
}
