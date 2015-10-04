#![allow(non_snake_case)] 
#![allow(non_upper_case_globals)] 

extern crate libc;
use std::mem;

extern {
  static _ZTVN10__cxxabiv117__class_type_infoE: u64;
  fn _ZN9IntHolderdlEPv(ptr: u64) -> ();
  fn _ZdlPv(ptr: u64) -> ();
  fn _ZdlPvm(ptr: u64, size: libc::size_t) -> ();
}

pub struct IntHolder {
  pub vt: &'static FooVTable,
  pub value: i64,
}

pub struct FooVTable {
  pub type_info: &'static TypeInfo,
  pub value: fn(&IntHolder) -> i64,
  pub complete_object_destructor: fn(&IntHolder)->(),
  pub deleting_destructor: fn(&IntHolder)->()

}

pub struct TypeInfo {
  pub vt: &'static u64,
  pub mangled_name: &'static str,
}

#[no_mangle]
pub static _ZTS9IntHolder: &'static str = "9LOLHolder\x00";

#[no_mangle]
pub static _ZTI9IntHolder: TypeInfo = TypeInfo {
  vt: &_ZTVN10__cxxabiv117__class_type_infoE,
  mangled_name: &_ZTS9IntHolder
};

#[no_mangle]
pub static _ZTV9IntHolder: FooVTable = FooVTable {
  type_info: &_ZTI9IntHolder,
  value: IntHolder::_ZN9IntHolder5valueEv,
  complete_object_destructor: IntHolder::_ZN9IntHolderD1Ev,
  deleting_destructor: IntHolder::_ZN9IntHolderD0Ev
};


impl IntHolder {

  #[no_mangle]
  pub fn _ZN9IntHolderC1El(&mut self, value: i64) {
    println!("rust >> IntHolder::IntHolder(long) C1");
    println!("self: {:p}", self as *const IntHolder);
    unsafe {
      let mut x : u64 = mem::transmute(&_ZTV9IntHolder);
      x += 0x8;
      self.vt = mem::transmute(x);
    }
    self.value = value;
  }

  #[no_mangle]
  pub fn _ZN9IntHolderC2ERKS_(&mut self, other: &IntHolder) {
    println!("rust >> IntHolder::IntHolder(IntHolder const&)");
    println!("self: {:p}", self as *const IntHolder);
    println!("other: {:p}", other as *const IntHolder);

    unsafe {
      let mut x : u64 = mem::transmute(&_ZTV9IntHolder);
      x += 0x8;
      self.vt = mem::transmute(x);
    }
    self.value = other.value;
    println!("self.value: {:?}", self.value);
    println!("other.value: {:?}", other.value);
  }

  #[no_mangle]
  pub fn _ZN9IntHolderC2El(&mut self, value: i64) {
    println!("rust >> IntHolder::IntHolder(long) C2");
    println!("self: {:p}", self as *const IntHolder);
    unsafe {
      let mut x : u64 = mem::transmute(&_ZTV9IntHolder);
      x += 0x8;
      self.vt = mem::transmute(x);
    }
    self.value = value;
  }

/* C3 constructor is never generally emitted by GCC/Clang
  #[no_mangle]
  pub fn _ZN9IntHolderC3El(&mut self, value: i64) {
    println!("rust >> IntHolder::IntHolder(long) C3");
  }
*/

  #[no_mangle]
  pub fn _ZN9IntHolderD0Ev(&self) {
    println!("rust >> IntHolder::~IntHolder() D0");
    println!("self: {:p}", self as *const IntHolder);

    IntHolder::_ZN9IntHolderD1Ev(self);
    unsafe {
      //static void operator delete(void*):
      //_ZN9IntHolderdlEPv(mem::transmute(self));

      //non-overridden delete:
      _ZdlPvm(mem::transmute(self),
        mem::size_of::<IntHolder>() as libc::size_t
      );
    }
  }

  #[no_mangle]
  pub fn _ZN9IntHolderD1Ev(&self) {
    println!("rust >> IntHolder::~IntHolder() D1");
    println!("self: {:p}", self as *const IntHolder);
  }


  #[no_mangle]
  pub fn _ZN9IntHolderD2Ev(&self) {
    println!("rust >> IntHolder::~IntHolder() D2");
  }

  #[no_mangle]
  pub fn _ZN9IntHolder5valueEv(&self) -> i64 {
    println!("rust >> IntHolder::value()");
    self.value
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3addEl(&mut self, other: i64) {
    println!("rust >> IntHolder::add(long)");
    self.value += other;
  }

  #[no_mangle]
  pub fn _ZN9IntHolder3subEl(&mut self, other: i64) {
    println!("rust >> IntHolder::sub(long)");
    self.value -= other;
  }

}

