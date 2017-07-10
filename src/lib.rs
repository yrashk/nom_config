// Copyright (c) 2017 Yurii Rashkovskii
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[allow(unused_imports)]
#[macro_use]
extern crate nom;

pub use nom::IResult;

use nom::{InputLength, InputIter, AsBytes, Slice};

/// Configuration container
pub struct Configured<T, Input>
    where Input : InputLength {
    config: T,
    input: Input,
}

impl<T, Input> Configured<T, Input>
    where Input : InputLength {

    /// Creates a new configuration container
    pub fn new(config: T, input: Input) -> Self {
        Configured { config, input, }
    }

    /// Returns a reference to the configuration
    pub fn config(&self) -> &T {
        &self.config
    }

    /// Returns a reference to the input
    pub fn input(&self) -> &Input {
        &self.input
    }

}

impl<T, Input> Into<(T, Input)> for Configured<T, Input>
    where Input : InputLength {
    fn into(self) -> (T, Input) {
        (self.config, self.input)
    }
}

impl<T, Input> InputLength for Configured<T, Input>
    where Input : InputLength {
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

impl<T, Input> InputIter for Configured<T, Input>
    where Input : InputLength + InputIter {
    type Item = Input::Item;
    type RawItem = Input::RawItem;
    type Iter = Input::Iter;
    type IterElem = Input::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::RawItem) -> bool {
        self.input.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Option<usize> {
        self.input.slice_index(count)
    }
}


use std::ops::{RangeFrom, RangeTo, Range};

impl<T, Input> Slice<RangeFrom<usize>> for Configured<T, Input>
    where Input : InputLength + Slice<RangeFrom<usize>>, T : Clone {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Configured::new(self.config.clone(), self.input.slice(range))
    }
}

impl<T, Input> Slice<RangeTo<usize>> for Configured<T, Input>
    where Input : InputLength + Slice<RangeTo<usize>>, T : Clone {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Configured::new(self.config.clone(), self.input.slice(range))
    }
}


impl<T, Input> Slice<Range<usize>> for Configured<T, Input>
    where Input : InputLength + Slice<Range<usize>>, T : Clone {
    fn slice(&self, range: Range<usize>) -> Self {
        Configured::new(self.config.clone(), self.input.slice(range))
    }
}

use std::ops::Deref;

impl<T, Input> Deref for Configured<T, Input>
    where Input : InputLength {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl<T, Input> Clone for Configured<T, Input>
    where Input : InputLength + Clone,
          T : Clone {
    fn clone(&self) -> Self {
        Configured::new(self.config.clone(), self.input.clone())
    }
}

impl<T, Input> AsBytes for Configured<T, Input>
    where Input : InputLength + AsBytes {
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

impl<T, Input> PartialEq<Configured<T, Input>> for Configured<T, Input>
    where Input : InputLength + PartialEq {
    fn eq(&self, other: &Configured<T, Input>) -> bool {
        self.input.eq(&other.input)
    }
}

impl<T, Input> Copy for Configured<T, Input>
    where Input : InputLength + Copy,
         T : Copy {}



use std::fmt::{Debug, Formatter, Result as FmtResult};
impl<T, Input> Debug for Configured<T, Input>
    where Input : InputLength + Debug,
          T : Debug {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("Config")
         .field("config", &self.config)
         .field("input", &self.input)
         .finish()
    }
}

#[macro_export]
/// Extracts a reference to the configuration
macro_rules! config {
    ($i:expr,) => {
       $crate::IResult::Done($i, $i.config())
    };
}

#[macro_export]
/// Lifts the configuration off the type for inner parsers
macro_rules! lift_config {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {{
        let (config, input) = $i.into();
        match $submac!(input, $($args)*) {
                $crate::IResult::Done(i, o) => $crate::IResult::Done($crate::Configured::new(config, i), o),
                $crate::IResult::Error(e) => $crate::IResult::Error(e),
                $crate::IResult::Incomplete(needed) => $crate::IResult::Incomplete(needed),
        }
    }};
    ($i:expr, $f:expr) => {
        lift_config!($i, call!($f));
    };
}

#[macro_export]
/// Defines a parser with a configuration
macro_rules! named_with_config {
    ($config:ty, $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables, unused_mut)]
    fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>,$o,u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        fn $name<'a>( mut i:  $crate::Configured<$config, &'a[u8]> ) -> $crate::IResult< $crate::Configured<$config, &'a [u8]>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, $name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        fn $name( mut i:  $crate::Configured<$config, &[u8]> ) -> $crate::IResult< $crate::Configured<$config, &[u8]>, &[u8], u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, pub $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        pub fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>,$o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, pub $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        pub fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, pub $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        pub fn $name( mut i: $crate::Configured<$config, $i> ) -> $crate::IResult<$crate::Configured<$config, $i>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        pub fn $name( mut i: $crate::Configured<$config, &[u8]> ) -> $crate::IResult<$crate::Configured<$config, &[u8]>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($config:ty, pub $name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables, unused_mut)]
        pub fn $name<'a>( mut i: $crate::Configured<$config, &'a [u8]>) -> $crate::IResult<$crate::Configured<$config, &'a [u8]>, &[u8], u32> {
            $submac!(i, $($args)*)
        }
    );
}


#[cfg(test)]
mod tests {
    use super::*;


    #[derive(Debug, Clone, Copy)]
    pub struct TestConfig(&'static [u8]);

    named_with_config!(TestConfig, test, do_parse!(cfg: config!() >> v: tag!(b"test") >> ({cfg.0})));
    named_with_config!(TestConfig, tests<Vec<&[u8]>>, many0!(alt!(lift_config!(tag!("skip")) | test)));

    #[test]
    fn it_works() {
        let it = test(Configured::new(TestConfig(b"\x00"), b"test"));
        assert_eq!(it.unwrap().1, b"\x00");

        let it = tests(Configured::new(TestConfig(b"\x00"), b"testskiptest"));
        assert_eq!(it.unwrap().1, vec!["\x00".as_bytes(), "skip".as_bytes(), "\x00".as_bytes()]);
    }
}
