# About
This is a rust library to derive "Builder" methods for arbitrary rust structs. It relies on the Macros-1.1 RFC and currently only works with the nightly rust build. 

# How to use
## Setup
In your ```Cargo.toml``` add
```toml
smelter = { git = "https://github.com/Ivan1931/smelter" }
```

At the top of your file add
```
#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;
```

Then just add ```[#derive(Builder)]``` above your struct,

## Example
```rust
#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;

#[test]
fn can_generate_builder_methods() {
    let point = Point::default().x(1).y_axis(2);
    let expected = Point {x: 1, y: 2};
    assert_eq!(point, expected);
}


#[derive(PartialEq, Debug, Builder, Default)]
struct Point {
    x: u32,
    #[field_name="y_axis"]
    y: u32,
}

#[test]
fn can_generate_builder_methods() {
    let point = Point::default()
                      .x(1)
                      .y_axis(2);
    let expected = Point {x: 1, y: 2};
    assert_eq!(point, expected);
}
```

## Code Generated by Example
```rust
# [ allow ( unused_attributes ) ]
# [ derive ( PartialEq , Debug , Default ) ]
struct Point {
    x: u32,
    # [ field_name = "y_axis" ]
    y: u32,
}

# [ allow ( dead_code ) ]
impl Point {
    fn x(self, __value: u32) -> Point {
        Point { x: __value, ..self }
    }
    
    fn y_axis(self, __value: u32) -> Point {
        Point { y: __value, ..self }
    }
    
    fn x_mut(&mut self, __value: u32) -> &mut Point {
        self.x = __value;
        self
    }
    
    fn y_axis_mut(&mut self, __value: u32) -> &mut Point {
        self.y = __value;
        self
    }
}
```
