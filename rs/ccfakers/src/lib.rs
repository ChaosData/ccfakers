#![allow(non_snake_case)]  

pub struct IntHolder {
  value: i64,
}

#[no_mangle]
pub fn _Z13new_IntHolderl(v: i64) -> Box<IntHolder> {
  Box::new(IntHolder { value: v })
}

#[no_mangle]
pub fn _Z16delete_IntHolderP9IntHolder(bih: Box<IntHolder>) {
  drop(bih);
}


impl IntHolder {

  #[no_mangle]
  pub fn _ZN9IntHolder5valueEv(&self) -> i64 {
    self.value
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3addEl(&mut self, other: i64) {
    self.value += other;
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3subEl(&mut self, other: i64) {
    self.value -= other;
  }

}

