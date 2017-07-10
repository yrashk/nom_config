# nom_config

nom_config is a small crate that allows to write [nom](https://github.com/Geal/nom) parser combinators while
carrying parsing configuration around.

Let's look at an artificial example. Suppose we want to pick up `"test"` tokens from an input, skipping
`"skip"` and replacing `"test"` with something we don't know upfront (a configurable replacement).
 
We can first designate a structure that will contain the replacement:

```rust
#[derive(Debug, Clone, Copy)]
struct Config {
    replacement: &'static [u8],
}

```

Now, we can use `named_with_config!` macro to pick up the `"test"` tag and retrieve the configuration using
the `config!` macro:

```rust
named_with_config!(Config, test, do_parse!(cfg: config!() >> v: tag!(b"test") >> ({cfg.replacement})));
```

For cases when you want to use parsers that are not aware of the configuration, you can use the
`lift_config!` macro:

```rust
named_with_config!(Config, tests<Vec<&[u8]>>, many0!(alt!(lift_config!(tag!("skip")) | test)));
```

You will typically need to do this when you have errors like this one:

```
   = note: expected type `nom::IResult<nom_config::Configured<_, &[u8]>, nom_config::Configured<Config, &[u8]>, _>`
              found type `nom::IResult<nom_config::Configured<_, &[u8]>, &[u8], _>`
```

Now, you can call your parser with your input and configuration wrapped into `Configured`: 

```rust
fn main() {
    let (_, result) = tests(Configured::new(Config { replacement: b"TEST" }, b"testskiptest")).unwrap();
    assert_eq!(result, vec!["TEST".as_bytes(), "skip".as_bytes(), "TEST".as_bytes()]);
}
```

As you can see, the `"test"` tag was replaced with `"TEST"`, as intended.

You can find the full example in the [examples](examples/replace.rs)

## License

This crate is licensed under the terms of the MIT license, same as nom. For details, see [LICENSE](LICENSE).

## Status

This crate is an early version. It is likely to be incomplete and have some bugs; it's also possible that a better
API can emerge later on.