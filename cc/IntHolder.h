#ifndef _INTHOLDER_H_
#define _INTHOLDER_H_

#include <stdint.h>

class IntHolder {
  public:
    IntHolder() = delete;
//    IntHolder(int64_t _value) = delete;
    int64_t value();
    void add(int64_t other);
    void sub(int64_t other);
  private:
    int64_t value_;
};

IntHolder* new_IntHolder(int64_t i);
void delete_IntHolder(IntHolder* ih);

#endif
