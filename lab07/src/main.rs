use {std::ops::Add, std::ops::Sub, std::ops::Mul, std::ops::Neg, std::fmt::Display};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Complex
{
    real : f64,
    imag : f64
}

impl Complex
{
    fn new<T, U>(new_real : T, new_imag : U) -> Self
    where
        f64 : From<T>,
        f64 : From<U>
    {
        Complex{real : f64::from(new_real), imag : f64::from(new_imag)}
    }

    fn conjugate(self) -> Self
    {
        Complex {real : self.real, imag : -self.imag}
    }
}

impl From<i32> for Complex
{
    fn from(value: i32) -> Self
    {
        Complex{real : f64::from(value), imag : 0.0}
    }
}

impl From<f64> for Complex
{
    fn from(value: f64) -> Self
    {
        Complex{real : value, imag : 0.0}   
    }
}

impl <T> Add<T> for Complex
where
    Complex : From<T>
{
    type Output = Complex;
    fn add(self, rhs : T) -> Self::Output
    {
        let rhs : Complex = Complex::from(rhs);
        Complex {real : self.real + rhs.real, imag : self.imag + rhs.imag}
    }
}

impl <T> Sub<T> for Complex
where
    Complex : From<T>
{
    type Output = Complex;
    fn sub(self, rhs : T) -> Self::Output
    {
        let rhs : Complex = Complex::from(rhs);
        Complex {real : self.real - rhs.real, imag : self.imag - rhs.imag}
    }
}

impl <T> Mul<T> for Complex
where
    Complex : From<T>
{
    type Output = Complex;
    fn mul(self, rhs : T) -> Self::Output
    {
        let rhs : Complex = Complex::from(rhs);
        Complex {real : self.real * rhs.real - self.imag * rhs.imag, imag : self.real * rhs.imag + self.imag * rhs.real}
    }
}

impl Neg for Complex
{
    type Output = Complex;
    fn neg(self) -> Self::Output
    {
        Complex {real : -self.real, imag : -self.imag}
    }
}

impl Display for Complex
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.real != 0.0 && self.imag != 0.0
        {
            if self.imag > 0.0
            {
                write!(f, "{}+{}i", self.real, self.imag)
            }
            else
            {
                write!(f, "{}{}i", self.real, self.imag)
            }
        }
        else if self.real == 0.0 && self.imag != 0.0
        {
            write!(f, "{}i", self.imag)
        }
        else if self.real != 0.0 && self.imag == 0.0
        {
            write!(f, "{}", self.real)
        }
        else
        {
            write!(f, "0")
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}