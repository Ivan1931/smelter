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

#[derive(Builder, PartialEq, Debug, Default)]
struct LotsOfFields<T: PartialEq> {
    pub this: String,
    structure: u32,
    has: i32,
    pub a: String,
    lot: T,
    pub of: &'static str,
    fields: String,
}

#[test]
fn public_fields_work() {
    let this = "this".to_string();
    let structure = 1u32;
    let has = -10i32;
    let a = "a".to_string();
    let lot = 7u32;
    let of = &"of";
    let fields = "fields".to_string();
    let expected: LotsOfFields<u32> = LotsOfFields {
        this: this.clone(),
        structure: structure.clone(),
        has: has.clone(),
        a: a.clone(),
        lot: lot.clone(),
        of: of,
        fields: fields.clone(),
    };
    let lof: LotsOfFields<u32> = LotsOfFields::default()
        .this(this)
        .structure(structure)
        .has(has)
        .a(a)
        .lot(lot)
        .of(of)
        .fields(fields);

    assert_eq!(lof, expected);
}

#[derive(Builder, PartialEq, Debug)]
struct WithLifeTime<'a> {
    l: &'a String,
}

#[test]
fn with_lifetime() {
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    let s3 = "world".to_string();
    let with_lifetime = WithLifeTime {
        l: &s1,
    };

    let expected = WithLifeTime {
        l: &s3,
    }.l(&s2);

    assert_eq!(with_lifetime, expected);
}

#[derive(PartialEq, Builder, Default, Debug, Clone)]
#[smelter(prefix="with_")]
pub struct User {
    pub uid: u64,
    pub email: String,
    pub alias: String,
    pub friends: Vec<User>,
}

#[test]
fn can_derive_collection() {
    let mut u1 = User::default();
    let u2 = User::default()
                .with_email("email@example.com".to_string())
                .with_alias("Ed".to_string())
                .with_uid(10u64);
    u1.with_email_mut("email@example.com".to_string())
      .with_alias_mut("Ed".to_string())
      .with_uid_mut(10u64);


    assert_eq!(u1, u2);
    let u3 = User::default().with_friends(vec![u1.clone(), u2.clone()]);
    assert_eq!(vec![u1, u2], u3.friends);

}
