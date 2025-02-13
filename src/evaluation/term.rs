use std::cell::RefCell;
use std::collections::LinkedList;

use crate::error::{Error, ErrorKind};
use crate::syntax::Symbol;

use super::combiner::NativeFn;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    has_value: bool,
    pub(crate) sub_terms: LinkedList<Term>,
    pub(crate) value: TermValue,
    pub(crate) value_ref: RefCell<TermValue>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermValue {
    Bool(BooleanValue),
    Int(i64),
    PrimitiveFn(NativeFn),
    Str(String),
    Sym(Symbol),
    Unit(UnitValue),
}

impl Term {
    pub fn new() -> Self {
        Self {
            has_value: false,
            sub_terms: LinkedList::new(),
            value: TermValue::Unit(UnitValue::Ignore),
            value_ref: RefCell::new(TermValue::Unit(UnitValue::Ignore))
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.sub_terms.is_empty()
    }

    pub fn len(&self) -> usize {
        self.sub_terms.len()
    }
}

impl Default for Term {
    fn default() -> Self {
        Term::new()
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.has_value {
            write!(f, "{:?}", self.sub_terms)
        } else {
            write!(f, "{:?}", self.value)
        }
    }
}

pub trait Access<T> {
    fn access(&self) -> &T;
}

pub trait AccessMut<T> {
    fn access_mut(&mut self) -> &mut T;
}

pub trait TryAccess<T> {
    fn try_access(&self) -> Result<&T, Error>;
}

pub trait TryAccessMut<T> {
    fn try_access_mut(&mut self) -> Result<&mut T, Error>;
}

pub trait TermAccess<T>:
    Access<T> + AccessMut<T> + TryAccess<T> + TryAccessMut<T> {}

macro_rules! impl_access {
    ($ty: ty, $ty_id: ident) => {
        impl Access<$ty> for Term {
            fn access(&self) -> &$ty {
                match self.value {
                    TermValue::$ty_id(ref val) => val,
                    _ => panic!()
                }
            }
        }

        impl AccessMut<$ty> for Term {
            fn access_mut(&mut self) -> &mut $ty {
                match self.value {
                    TermValue::$ty_id(ref mut val) => val,
                    _ => panic!()
                }
            }
        }

        impl TryAccess<$ty> for Term {
            fn try_access(&self) -> Result<&$ty, Error> {
                match self.value {
                    TermValue::$ty_id(ref val) => Ok(val),
                    // TODO: Add messages.
                    _ => Err(Error::new(ErrorKind::TypeMismatch))
                }
            }
        }

        impl TryAccessMut<$ty> for Term {
            fn try_access_mut(&mut self) -> Result<&mut $ty, Error> {
                match self.value {
                    TermValue::$ty_id(ref mut val) => Ok(val),
                    _ => Err(Error::new(ErrorKind::TypeMismatch))
                }
            }
        }

        impl TermAccess<$ty> for Term {}

        impl From<$ty> for Term {
            fn from(value: $ty) -> Term {
                let mut term = Term::new();
                term.has_value = true;
                term.value = TermValue::$ty_id(value);
                term
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitValue {
    Ignore
}

type BooleanValue = bool;

impl_access!(BooleanValue, Bool);
impl_access!(i64, Int);
impl_access!(NativeFn, PrimitiveFn);
impl_access!(UnitValue, Unit);
impl_access!(String, Str);
impl_access!(Symbol, Sym);
