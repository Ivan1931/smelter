/*!
# Introduction
Smelter is a proc macro that automatically derives methods for a builder
pattern from a struct. 
Adding `#[derive(Builder)]` above your struct will instruct smelter to
automatically generate a "builder" method for each field.
For example:

```
#[derive(Builder)]
struct User {
    alias: String,
    email: String,
    uid: u64,
}
```

Allows you to call

```
// new method is *not* derived by this macro.
let p = User::new() 
            .alias(some_alias_string)
            .email(some_email_string)
            .uid(some_uid);
```

# Setup
First add smelter as a dependency in your `Cargo.toml`:

```toml
[dependencies]
smelter = "*"
```

Add the following to your `lib.rs` or `main.rs` file:

```
#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;
```

Then just add `#[derive(Builder)]` above your struct,


# Simple Example
This example illustrates what code get generated:

```
#[derive(Builder)]
struct Point {
    x: u32,
    y: u32,
}
```
Generates the methods:

```
impl Point {
    pub fn x(self, __value: u32) -> Point {
        Point { x: __value, ..self }
    }
    
    pub fn y(self, __value: u32) -> Point {
        Point { y: __value, ..self }
    }
    
    pub fn x_mut(&mut self, __value: u32) -> &mut Point {
        self.x = __value;
        self
    }
    
    pub fn y_mut(&mut self, __value: u32) -> &mut Point {
        self.y = __value;
        self
    }
}
```
# Custom Prefix
A prefix can be added before all generated methods.
## Example
```
#[derive(Builder)]
#[smelter(prefix="with_")]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
```
Will generate:

```
impl Point {
    pub fn with_x(self, __value: u32) -> Point {
        Point { x: __value, ..self }
    }
    
    pub fn with_y(self, __value: u32) -> Point {
        Point { y: __value, ..self }
    }
    
    pub fn with_x_mut(&mut self, __value: u32) -> &mut Point {
        self.x = __value;
        self
    }
    
    pub fn with_y_mut(&mut self, __value: u32) -> &mut Point {
        self.y = __value;
        self
    }
}
```
# Custom fields
It's also possible to change the name of the generated method by placing
the following attribute above the field declaration:
## Example
```
#[derive(PartialEq, Debug, Builder, Default)]
struct container<T> 
    where T: partialeq  + default {
    #[smelter(field_name="i")]
    item: T,
}
```
Will generate:

```
impl<T> Container<T>
    where T: PartialEq + Default
{
    fn i(self, __value: T) -> Container<T> {
        Container { item: __value, ..self }
    }

    fn i_mut(&mut self, __value: T) -> &mut Container<T> {
        self.item = __value;
        self
    }
}
```

## Note
If combined with the `prefix` attribute the prefix, will be
appended onto the front of the custom field name.

# Visibility
By default `#[derive(Builder)]` will mimic the visibility of fields in your struct.
For example if some field `p` is public (has a `pub` identifier), the generated `p` and `p_mut` builder methods will be public.

## Example
```
pub struct AssymetricKeyPair<A, B> {
    pub public_key: A,
    private_key: B,
}
```

Will generate the code:

```
impl<A, B> AssymetricKeyPair<A, B> {
    pub fn public_key(self, __value: A) -> AssymetricKeyPair<A, B> {
        AssymetricKeyPair { public_key: __value, ..self }
    }

    fn private_key(self, __value: B) -> AssymetricKeyPair<A, B> {
        AssymetricKeyPair { private_key: __value, ..self }
    }

    pub fn public_key_mut(&mut self, __value: A) -> &mut AssymetricKeyPair<A, B> {
        self.public_key = __value;
        self
    }

    fn private_key_mut(&mut self, __value: B) -> &mut AssymetricKeyPair<A, B> {
        self.private_key = __value;
        self
    }
}
```

# Caveats
* At the moment, builder methods for enums, tuple structs and unit structs cannot be generated.
* The library makes use of [macros 1.1](https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md) a feature, that is currenly only available for [nighlty builds](https://doc.rust-lang.org/book/nightly-rust.html).
*/
#![feature(proc_macro, proc_macro_lib)]
#![cfg(not(test))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
mod code_gen;

#[proc_macro_derive(Builder)]
/**
 # Arguments
 * `input` stream of tokens obtained from the compiler.
 
 # Returns
 * A `TokenStream` containing the origional struct definition and expanded `impl` methods.
 
 # Panics
 * `#[derive(Builder)]` is invoked for an enum, unit struct or tuple struct.
 * Arguments to `#[smelter(field_name="..")]` or `#[smelter(prefix="..")]` are not strings.
 * An empty `field_name` is specified. IE: `#[smelter(field_name="")]`
 */
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_macro_input(&source).unwrap();

    let expanded = code_gen::expand_builder(ast);

    expanded.to_string().parse().unwrap()
}

