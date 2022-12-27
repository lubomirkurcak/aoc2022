use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

pub enum Expression<T> {
    Add(Box<Expression<T>>, Box<Expression<T>>),
    Sub(Box<Expression<T>>, Box<Expression<T>>),
    Mul(Box<Expression<T>>, Box<Expression<T>>),
    Div(Box<Expression<T>>, Box<Expression<T>>),
    Eq(Box<Expression<T>>, Box<Expression<T>>),
    Ident(String),
    Const(T),
    Free,
}

impl<T> Expression<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: FromStr,
    T: Copy,
    T: PartialEq,
    T: From<bool>,
    T: Debug,
{
    pub fn eval(&self, vals: &HashMap<String, Expression<T>>) -> Result<T, ()> {
        match self {
            Expression::Add(a, b) => Ok(a.eval(vals)? + b.eval(vals)?),
            Expression::Sub(a, b) => Ok(a.eval(vals)? - b.eval(vals)?),
            Expression::Mul(a, b) => Ok(a.eval(vals)? * b.eval(vals)?),
            Expression::Div(a, b) => Ok(a.eval(vals)? / b.eval(vals)?),
            Expression::Eq(a, b) => Ok((a.eval(vals)? == b.eval(vals)?).into()),
            Expression::Ident(ident) => Ok(vals.get(ident).unwrap().eval(vals)?),
            Expression::Const(val) => Ok(*val),
            Expression::Free => Err(()),
        }
    }

    pub fn force_result(
        &self,
        result: T,
        vals: &HashMap<String, Expression<T>>,
    ) -> HashMap<String, T> {
        let mut forced = HashMap::new();
        self.force_result_internal(None, result, vals, &mut forced);
        forced
    }

    fn force_result_internal(
        &self,
        my_ident: Option<&str>,
        result: T,
        vals: &HashMap<String, Expression<T>>,
        forced: &mut HashMap<String, T>,
    ) {
        match self {
            Expression::Add(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a + b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = result - a_val.unwrap();
                    assert_eq!(result, a_val.unwrap() + b_val);
                    b.force_result_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result - b_val.unwrap();
                    assert_eq!(result, a_val + b_val.unwrap());
                    a.force_result_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expression::Sub(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a - b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = a_val.unwrap() - result;
                    assert_eq!(result, a_val.unwrap() - b_val);
                    b.force_result_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result + b_val.unwrap();
                    assert_eq!(result, a_val - b_val.unwrap());
                    a.force_result_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expression::Mul(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a * b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = result / a_val.unwrap();
                    assert_eq!(result, a_val.unwrap() * b_val);
                    b.force_result_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result / b_val.unwrap();
                    assert_eq!(result, a_val * b_val.unwrap());
                    a.force_result_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expression::Div(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a / b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = a_val.unwrap() / result;
                    assert_eq!(result, a_val.unwrap() / b_val);
                    b.force_result_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result * b_val.unwrap();
                    assert_eq!(result, a_val / b_val.unwrap());
                    a.force_result_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expression::Eq(a, b) => {
                // NOTE(lubo): Only enforcing equality is supported!
                assert_eq!(result, true.into());

                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a == b
                if a_val.is_ok() && b_val.is_err() {
                    b.force_result_internal(None, a_val.unwrap(), vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    a.force_result_internal(None, b_val.unwrap(), vals, forced);
                } else {
                    panic!();
                }
            }
            Expression::Ident(ident) => {
                vals.get(ident)
                    .unwrap()
                    .force_result_internal(Some(ident), result, vals, forced);
            }
            Expression::Const(c) => {
                assert_eq!(c, &result);
            }
            Expression::Free => {
                assert!(my_ident.is_some());
                let my_ident = my_ident.unwrap().to_string();
                if forced.contains_key(&my_ident) {
                    assert_eq!(forced.get(&my_ident).unwrap(), &result);
                } else {
                    forced.insert(my_ident, result);
                }
            }
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.trim();

        if let Some(i) = s.find('=') {
            let split = s.split_at(i);
            return Self::Eq(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if let Some(i) = s.find('+') {
            let split = s.split_at(i);
            return Self::Add(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if let Some(i) = s.find('-') {
            let split = s.split_at(i);
            return Self::Sub(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if let Some(i) = s.find('*') {
            let split = s.split_at(i);
            return Self::Mul(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if let Some(i) = s.find('/') {
            let split = s.split_at(i);
            return Self::Div(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        match s.parse::<T>() {
            Ok(val) => Self::Const(val),
            Err(_) => Self::Ident(s.to_string()),
        }
    }
}
