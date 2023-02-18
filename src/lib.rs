use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 一个代表3维向量的类型.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {

    /// 一些常用的的向量类型
    pub const ZERO: Self = Self::fill(0.0);
    pub const ONE: Self = Self::fill(1.0);
    pub const NEG: Self = Self::fill(-1.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const NAN: Self = Self::fill(f32::NAN);
    
    // Util：方法
    /// 构建一个新的Vec3并用给定的值填充每个元素
    pub const fn fill(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Vec3的构造函数，用x, y, z 3个浮点数来构造一个新的向量
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        // println!("Vec3::new(xyz) Called!");
        Self { x, y, z }
    }

    /// 标准化3维向量的方法
    /// 向量不能为0，或者很接近0
    pub fn norm(self) -> Self {
        // 除以模等于乘以模的倒数
        let mag_recip = self.mag().recip();
        if mag_recip.is_finite() && mag_recip>0.0 {
            // 我们已经实现了Vec3的运算符重载 这里直接可以使用乘法符号进行运算
            self * mag_recip
        }else {
            Self::ZERO
        }
    }

    /// 计算当前向量和给定向量的叉积
    /// 向量a和向量b的叉积的结果是一个新的向量c
    /// $c_x = a_yb_z − a_zb_y$
    /// $c_y = a_zb_x − a_xb_z$
    /// $c_z = a_xb_y − a_yb_x$
    pub fn cross(&self, v: &Vec3) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    /// 求两个Vecs之间的距离
    /// d = \sqrt{(b_×-a_x)^2 + (b_y-a_y)^2 + (b_z-a_z)^2}
    pub fn dist(self, b: Vec3) -> f32 {
        ((b.x-self.x).powf(2.0) + (b.y-self.y).powf(2.0) + (b.z-self.z).powf(2.0)).sqrt()
    }

    /// 第二种方法
    pub fn dist2(self, b: Vec3) -> f32 {
        let s = b - self;
        s.dot(&s).sqrt()
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

    // Vec3标准化
    #[test]
    fn test_norm() {
        let v1 = Vec3::new(12.0, -5.0, 0.0);
        println!("v1.norm(): {:?}", v1.norm());
        assert_eq!(v1.norm(), Vec3::new(0.923077, -0.3846154, 0.0));
    }

    // 求两个Vecs之间的距离
    #[test]
    fn test_dist() {
        let v1 = Vec3::new(3.0, -6.0, 3.0);
        let v2 = Vec3::new(5.0, 7.0, -4.0);
        println!("v1.dist(v2): {:?}", v1.dist(v2));
        assert_eq!(v1.dist(v2), 222.0_f32.sqrt());
    }
    
    // Vec3差积运算
    #[test]
    fn test_cross() {
        let v1 = Vec3::new(3.0, 6.0, 2.0);
        let v2 = Vec3::new(5.0, 7.0, 1.0);
        // 两个Vec3求点积运算
        println!("v2.dot(&v2): {:?}", v2.cross(&v1));
        assert_eq!(v1.cross(&v2), Vec3::new(-8.0, 7.0, -9.0));
    }

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
        println!("v1.dot(&v2) -> {}", v1.dot(&v2));
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
        // Vec3与Vec3加法运算测试
        assert_eq!(v1 + v2, Vec3::new(2.0, 3.0, 4.0));
        // Vec3与Vec3加法赋值运算测试
        v1 += v2;
        assert_eq!(v1, Vec3::new(2.0, 3.0, 4.0));
        // Vec3与f32加法赋值运算测试
        v1 += 0.1;
        assert_eq!(v1, Vec3::new(2.1, 3.1, 4.1));
    }

    #[test]
    fn test_fill_vec3() {
        let mut v1 = Vec3::fill(1.0);
        let v2 = Vec3::fill(2.0);
        assert_eq!(v1 + v2, Vec3::new(3.0, 3.0, 3.0));
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
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1, Vec3::fill(1.0));
        // 拷贝构造 Vec3
        let mut v2 = Vec3::from(v1);
        v2.x = 3.0;
        assert_eq!(v2.x, 3.0);
        // Vec3取负
        v2 = -v2;
        print!("{:#?}", v2);
        assert_eq!(v2, Vec3::new(-3.0, -1.0, -1.0));
        // Vec3置零
        v2.zero();
        assert_eq!(v2, Vec3::fill(0.0));
    }
}
