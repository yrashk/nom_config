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

#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_config;

use nom_config::Configured;

#[derive(Debug, Clone, Copy)]
struct Config {
    replacement: &'static [u8],
}

named_with_config!(Config, test, do_parse!(cfg: config!() >> v: tag!(b"test") >> ({cfg.replacement})));
named_with_config!(Config, tests<Vec<&[u8]>>, many0!(alt!(lift_config!(tag!("skip")) | test)));


fn main() {
    let (_, result) = tests(Configured::new(Config { replacement: b"TEST" }, b"testskiptest")).unwrap();
    assert_eq!(result, vec!["TEST".as_bytes(), "skip".as_bytes(), "TEST".as_bytes()]);
}