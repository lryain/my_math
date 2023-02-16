use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 一个代表3维向量的类型.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    /// Vec3的构造函数，用x, y, z 3个浮点数来构造一个新的向量
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        // println!("Vec3::new(xyz) Called!");
        Self { x, y, z }
    }

    /// 向量的点积,也叫向量的内积、数量积，对两个向量执行点乘运算，
    /// 就是对这两个向量对应位一一相乘之后求和的操作，点乘的结果是一个标量。
    /// 采取第二种求模公式：
    /// a · b = a_x × b_x + a_y × b_y + a_z × b_z
    /// 
    /// 取得当前向量和给定的向量的点积的方法
    pub fn dot(&self, v: &Self) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    /// 取得当前向量的模magnitude的方法
    /// 也称作求向量的长度，大小
    pub fn mag(&self) -> f32 {
        self.dot(&self).sqrt()
    }

    // 将当前向量置0的方法
    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

/// 用给定的Vec3来构造新的Vec3向量
/// C++中也叫拷贝构造函数
impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

/// 运算符重载实现Vec3与Vec3的加法运算
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

/// 运算符重载实现Vec3与f32的加法运算
impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x.add(v),
            y: self.y.add(v),
            z: self.z.add(v),
        }
    }
}

/// 运算符重载实现Vec3与Vec3的加法赋值运算
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x.add_assign(v.x);
        self.y.add_assign(v.y);
        self.z.add_assign(v.z);
    }
}

/// 运算符重载实现Vec3与f32的加法赋值运算
impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, v: f32) {
        self.x.add_assign(v);
        self.y.add_assign(v);
        self.z.add_assign(v);
    }
}

/// 运算符重载实现Vec3与Vec3的减法运算
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

/// 运算符重载实现Vec3与f32的减法运算
impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x.sub(v),
            y: self.y.sub(v),
            z: self.z.sub(v),
        }
    }
}

/// 运算符重载实现Vec3与Vec3的减法赋值运算
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.x.sub_assign(v.x);
        self.y.sub_assign(v.y);
        self.z.sub_assign(v.z);
    }
}

/// 运算符重载实现Vec3与f32的减法赋值运算
impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, v: f32) {
        self.x.sub_assign(v);
        self.y.sub_assign(v);
        self.z.sub_assign(v);
    }
}

/// 运算符重载实现Vec3与Vec3的乘法运算
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

/// 运算符重载实现Vec3与f32的乘法运算
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x.mul(v),
            y: self.y.mul(v),
            z: self.z.mul(v),
        }
    }
}

/// 运算符重载实现Vec3与Vec3的乘法赋值运算
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        self.x.mul_assign(v.x);
        self.y.mul_assign(v.y);
        self.z.mul_assign(v.z);
    }
}

/// 运算符重载实现Vec3与f32的乘法赋值运算
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, v: f32) {
        self.x.mul_assign(v);
        self.y.mul_assign(v);
        self.z.mul_assign(v);
    }
}

/// 运算符重载实现Vec3与Vec3的除法运算
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

/// 运算符重载实现Vec3与f32的除法运算
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x.div(v),
            y: self.y.div(v),
            z: self.z.div(v),
        }
    }
}

/// 运算符重载实现Vec3与Vec3的除法赋值运算
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        self.x.div_assign(v.x);
        self.y.div_assign(v.y);
        self.z.div_assign(v.z);
    }
}

/// 运算符重载实现Vec3与f32的除法赋值运算
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, v: f32) {
        self.x.div_assign(v);
        self.y.div_assign(v);
        self.z.div_assign(v);
    }
}

/// 负号操作符重载
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
    // 导入这个模块的全部类型
    use super::*;

    // Vec3点积运算
    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        // 当前vec3求点积
        assert_eq!(v1.dot(&v1), 3.0);
        assert_eq!(v2.dot(&v2), 14.0);
        // 两个Vec3求点积运算
        assert_eq!(v1.dot(&v2), 6.0);
    }

    // Vec3求模运算
    #[test]
    fn test_mag() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        // println!("{:#?}", v1);
        // println!("v1.mag:[{}] v1.dot:[{}]", v1.mag(), v1.dot(&v1).sqrt());
        assert_eq!(v1.mag(), v1.dot(&v1).sqrt());
    }

    // Vec3加法运算
    #[test]
    fn test_vec3_add() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 + v2, Vec3::new(2.0, 3.0, 4.0));
        // 赋值运算
        v1 += v2;
        assert_eq!(v1, Vec3::new(2.0, 3.0, 4.0));
        v1 += 0.1;
        assert_eq!(v1, Vec3::new(2.1, 3.1, 4.1));
    }

    // Vec3减法运算
    #[test]
    fn test_vec3_sub() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 - v2, Vec3::new(0.0, 1.0, 2.0));
        // 赋值运算
        v1 -= v2;
        assert_eq!(v1, Vec3::new(0.0, 1.0, 2.0));
        v1 -= 0.1;
        assert_eq!(v1, Vec3::new(-0.1, 0.9, 1.9));
    }

    // Vec3乘法运算
    #[test]
    fn test_vec3_mul() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v1 * v2, Vec3::new(2.0, 4.0, 6.0));
        // 赋值运算
        v1 *= v2;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(4.0, 8.0, 12.0));
    }

    // Vec3除法运算
    #[test]
    fn test_vec3_div() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 / v2, Vec3::new(1.0, 2.0, 3.0));
        // 赋值运算
        v1 /= v2;
        assert_eq!(v1, Vec3::new(1.0, 2.0, 3.0));
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 1.5));
    }

    // 用给定Vec3构造一个新的Vec3
    #[test]
    fn test_from() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let mut v2 = Vec3::from(v);
        v2.x = 3.0;
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2, Vec3::new(3.0, 1.0, 1.0))
    }

    // 对当前Vec3取负值运算
    #[test]
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
