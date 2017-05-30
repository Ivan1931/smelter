[![Build Status](https://travis-ci.org/Ivan1931/smelter.svg?branch=master)](https://travis-ci.org/Ivan1931/smelter)
[![Latest Version](https://img.shields.io/crates/v/smelter.svg)](https://crates.io/crates/smelter)

# About
This is a rust library to derive "Builder" methods for arbitrary rust structs. It relies on the Macros-1.1 RFC and currently only works with the nightly rust build. 

# Usage
## Setup

```toml
[dependencies]
smelter = "*"
```

Add the following to the file in which you with to use smelter.

```
#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;
```

Then just add ```#[derive(Builder)]``` above your struct,

## Example
<table border="0">
<tr>
<td>
Smelter
</td>
<td>
Generated Code
</td>
</tr>
<tr>
<td>
<pre lang="rust">

#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;

#[derive(PartialEq, Builder, Default, Debug, Clone)]
#[smelter(prefix="with_")]
pub struct User {
    pub uid: u64,
    pub email: String,
    pub alias: String,
    pub friends: Vec<User>,
}

</pre>

</td>

<td>
<pre lang="rust">
# [ allow ( dead_code ) ]
impl User {
    pub fn with_uid(self, __value: u64) -> User {
        User { uid: __value, ..self }
    }
    pub fn with_email(self, __value: String) -> User {
        User { email: __value, ..self }
    }
    pub fn with_alias(self, __value: String) -> User {
        User { alias: __value, ..self }
    }
    pub fn with_friends(self, __value: Vec<User>) -> User {
        User { friends: __value, ..self }
    }
    pub fn with_uid_mut(&mut self, __value: u64) -> &mut User {
        self.uid = __value;
        self
    }
    pub fn with_email_mut(&mut self, __value: String) -> &mut User {
        self.email = __value;
        self
    }
    pub fn with_alias_mut(&mut self, __value: String) -> &mut User {
        self.alias = __value;
        self
    }
    pub fn with_friends_mut(&mut self, __value: Vec<User>) -> &mut User {
        self.friends = __value;
        self
    }
}
</pre>
</td>
</tr>
</table>


### Invocation

```rust
// ... somewhere in your code
     let mut u1 = User::default();
// ...
     u1.with_email_mut("cardboardbox@example.com".to_string())
      .with_alias_mut("Cardboard box".to_string())
      .with_uid_mut(10u64);
      
// ... somewhere else
    let u2 = User::default()
                .with_email("filecabinate@example.com".to_string())
                .with_alias("File Cabinate".to_string())
                .with_uid(10u64);
```

## More Examples
For more examples see the [test.rs](https://github.com/Ivan1931/smelter/blob/master/tests/test.rs)

# Caveats
* Currently this library will only work on a nightly build of rust.

* It relies on [macros 1.1](https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md)

* Will not work for enums, unit structs and tuple structs. 
