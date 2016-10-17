#![feature(proc_macro, custom_attribute)]

#[macro_use]
extern crate smelter;

#[derive(PartialEq, Debug, Builder, Default)]
struct Point {
    x: u32,
    #[smelter(field_name="y_axis")]
    y: u32,
}

#[derive(PartialEq, Debug, Builder, Default)]
struct Container<T> 
    where T: PartialEq  + Default {
    item: T,
}

#[test]
fn can_generate_builder_methods() {
    let point = Point::default().x(1).y_axis(2);
    let expected = Point {x: 1, y: 2};
    assert_eq!(point, expected);
}

#[test]
fn can_generate_generic_builder_methods() {
    let container: Container<u32> = Container::default().item(1u32);
    let expected = Container { item: 1u32 };
    assert_eq!(container, expected);
}

#[test]
fn can_generate_mutable_methods() {
    let mut point = Point::default();
    point.x_mut(1).y_axis_mut(2);
    let expected = Point { x: 1, y: 2};
    assert_eq!(point, expected);
}

#[derive(PartialEq, Builder, Default, Debug)]
#[smelter(prefix="with_")]
struct ContainerWith<T>
    where T: PartialEq + Default {
        item: T,
        #[smelter(field_name = "id")]
        item_id: u64,
}

#[test]
fn can_generate_container_with_prefix() {
    let container: ContainerWith<u32> = ContainerWith::default()
                                            .with_item(1u32)
                                            .with_id(5u64);
    let expected = ContainerWith { item: 1u32 , item_id: 5u64 };
    assert_eq!(container, expected);
}
