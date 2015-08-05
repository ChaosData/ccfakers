#ifndef _INTHOLDER_H_
#define _INTHOLDER_H_

#include <stdint.h>
#include <stdio.h>

class IntHolder {
  public:
    IntHolder(int64_t _value);
    virtual int64_t value();
    void add(int64_t other);
    void sub(int64_t other);
    virtual ~IntHolder();
  private:
    int64_t value_;
};

#endif
