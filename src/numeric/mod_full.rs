mod modulo {
    // const MOD: i32 = 998244353;
    const MOD: i32 = 1e9 as i32 + 7;

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mod(i32);

    impl Mod {
        #[allow(unused)]
        pub const ZERO: Self = Self(0);

        #[allow(unused)]
        pub const ONE: Self = Self(1);

        fn rev_rec(a: i32, m: i32) -> i32 {
            if a == 1 {
                return a;
            }
            return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
        }

        #[allow(dead_code)]
        fn inv(self) -> Mod {
            Mod(Self::rev_rec(self.0, MOD))
        }
    }

    impl From<i32> for Mod {
        fn from(mut x: i32) -> Self {
            x += if x < 0 { MOD } else if x >= MOD { -MOD } else { 0 };
            assert!(0 <= x && x < MOD);
            Self(x)
        }
    }

    impl std::fmt::Display for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::fmt::Debug for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            const MAX: usize = 100;
            if self.0 <= MAX as i32 {
                write!(f, "{}", self.0)
            } else if self.0 >= MOD - MAX as i32 {
                write!(f, "-{}", MOD - self.0)
            } else {
                for denum in 1..MAX {
                    for num in 1..MAX {
                        if Mod(num as i32) / Mod(denum as i32) == *self {
                            return write!(f, "{}/{}", num, denum);
                        }
                    }
                }
                write!(f, "(?? {} ??)", self.0)
            }
        }
    }

    impl std::ops::Add for Mod {
        type Output = Mod;
        fn add(self, rhs: Self) -> Self::Output {
            let res = self.0 + rhs.0;
            if res >= MOD {
                Mod(res - MOD)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            if self.0 >= MOD {
                self.0 -= MOD;
            }
        }
    }

    impl std::ops::Sub for Mod {
        type Output = Mod;
        fn sub(self, rhs: Self) -> Self::Output {
            let res = self.0 - rhs.0;
            if res < 0 {
                Mod(res + MOD)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
            if self.0 < 0 {
                self.0 += MOD;
            }
        }
    }

    impl std::ops::Mul for Mod {
        type Output = Mod;
        fn mul(self, rhs: Self) -> Self::Output {
            let res = ((self.0 as i64) * (rhs.0 as i64)) % (MOD as i64);
            Mod(res as i32)
        }
    }

    impl std::ops::MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = (((self.0 as i64) * (rhs.0 as i64)) % (MOD as i64)) as i32;
        }
    }

    impl std::ops::Div for Mod {
        type Output = Mod;
        fn div(self, rhs: Self) -> Self::Output {
            let rhs_inv = rhs.inv();
            self * rhs_inv
        }
    }

    impl std::ops::DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.inv();
        }
    }
}
// Usage:
// let x = Mod::from(200);
// let y = Mod::from(100);
// let ans = (x+y) + (x-y) + (x*y) + (x/y); 