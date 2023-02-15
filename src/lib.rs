
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 3-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    /// 用x, y, z来构造一个向量
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        // println!("Vec3::new Called!");
        Self { x, y, z }
    }

    // Zero
    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

/// 用Vec3来构造一个向量
impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

/// 在Rust中数值类型的加减乘除运算可以通过实现运算符重载来实现，下面先看一个加法的例子：
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x.add(v.x),
            y: self.y.add(v.y),
            z: self.z.add(v.z),
        }
    }
}

// Add的另外一种写法
// impl Add<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn add(self, v: Vec3) -> Self::Output {
//         Vec3 {
//             x: self.x + v.x,
//             y: self.y + v.y,
//             z: self.z + v.z,
//         }
//     }
// }

/// 通过实现运算符重载来实现Vec3减法
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x.sub(v.x),
            y: self.y.sub(v.y),
            z: self.z.sub(v.z),
        }
    }
}

/// 通过实现运算符重载来实现Vec3减法
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x.mul(v.x),
            y: self.y.mul(v.y),
            z: self.z.mul(v.z),
        }
    }
}

/// 通过实现运算符重载来实现Vec3减法
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x.div(v.x),
            y: self.y.div(v.y),
            z: self.z.div(v.z),
        }
    }
}

/// 操作符重载
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(), // 调用标准库的宏生成 -号的实现
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 加法测试
    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 + v2, Vec3::new(2.0, 3.0, 4.0))
    }

    // 减法测试
    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 - v2, Vec3::new(0.0, 1.0, 2.0))
    }

    // 乘法测试
    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v1 * v2, Vec3::new(2.0, 4.0, 6.0))
    }

    // 除法测试
    #[test]
    fn test_vec3_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 / v2, Vec3::new(1.0, 2.0, 3.0))
    }

    // 用给定Vec3构造一个新的Vec3
    #[test]
    fn test_from() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let mut v2 = Vec3::from(v);
        v2.x = 3.0;
        assert_eq!(v2, Vec3::new(3.0, 1.0, 1.0))
    }

    // 对当前Vec3取负值
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn it_works() {
        // 用给定的值构造一个新的Vec3
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);

        let mut v2 = Vec3::from(v);
        v2.x = 3.0;
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 1.0);
        assert_eq!(v2.z, 1.0);
        v2 = -v2;
        assert_eq!(v2.x, -3.0);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, -1.0);
        print!("{:#?}", v2);
        v2.zero();
        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
    }
}
