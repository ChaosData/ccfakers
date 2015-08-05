#![allow(non_snake_case)] 
#![allow(non_upper_case_globals)] 

extern crate libc;
use std::mem;


extern {
  static _ZTVN10__cxxabiv117__class_type_infoE: u64;
}

pub struct IntHolder {
  pub vt: &'static FooVTable,
  pub value: i64,
}

pub struct FooVTable {
  pub type_info: &'static TypeInfo,
  pub value: fn(&IntHolder) -> i64,
  pub pad: u64,
  pub destructor: fn(&IntHolder)->(),
}

pub struct TypeInfo {
  pub vt: &'static u64,
  pub mangled_name: &'static str,
}

static name: &'static str = "9LOLHolder\x00";

pub static FooTV: TypeInfo = TypeInfo {
  vt: &_ZTVN10__cxxabiv117__class_type_infoE,
  mangled_name: &name
};

#[no_mangle]
pub static _ZTV9IntHolder: FooVTable = FooVTable {
  type_info: &FooTV,
  value: IntHolder::_ZN9IntHolder5valueEv,
  pad: 0,
  destructor: IntHolder::_ZN9IntHolderD0Ev
};


fn puts(s: &str) {
  unsafe {
    libc::puts(mem::transmute(s.as_ptr()));
  }
}

impl IntHolder {

  #[no_mangle]
  pub fn _ZN9IntHolderC1El(&mut self, value: i64) {
    puts("rust >> IntHolder::IntHolder(long)");
    unsafe {
      let mut x : u64 = mem::transmute(&_ZTV9IntHolder);
      x += 0x8;
      self.vt = mem::transmute(x);
    }
    self.value = value;
  }

  #[no_mangle]
  pub fn _ZN9IntHolderD0Ev(&self) {
    puts("rust >> IntHolder::~IntHolder()");
    unsafe {
      libc::free( mem::transmute(self) );
    }
  }

  #[no_mangle]
  pub fn _ZN9IntHolder5valueEv(&self) -> i64 {
    puts("rust >> IntHolder::value()");
    self.value
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3addEl(&mut self, other: i64) {
    puts("rust >> IntHolder::add(long)");
    self.value += other;
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3subEl(&mut self, other: i64) {
    puts("rust >> IntHolder::sub(long)");
    self.value -= other;
  }

}

