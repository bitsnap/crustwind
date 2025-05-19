use crustwind_macro::css_unit;
use derive_more::*;
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

css_unit!(Em, "em");
css_unit!(Pct, "%");
css_unit!(Px, "px");
css_unit!(Rem, "rem");
css_unit!(Vw, "vw");
css_unit!(Vh, "vh");
css_unit!(Vmin, "vmin");
css_unit!(Vmax, "vmax");

fn main() {
    println!("{}", Em(1));
    println!("{}", Pct(2));
    println!("{}", Px(3));
    println!("{}", Rem(4));
    println!("{}", Vw(5));
    println!("{}", Vh(6));
    println!("{}", Vmin(7));
    println!("{}", Vmax(8));
}
