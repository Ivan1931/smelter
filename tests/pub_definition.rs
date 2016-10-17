#![feature(proc_macro, custom_derive, custom_attribute)]

#[derive(Builder, PartialEq, Default, Debug)]
pub struct PubTest {
    pub s: String,
    #[smelter(force_public)]
    r: u32,
}
