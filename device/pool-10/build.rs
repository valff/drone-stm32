#[macro_use]
extern crate drone_stm32_svd;

fn main() {
  drone_stm32_svd::generate_reg_map(svd_feature!(), Some((10, 12)));
}
