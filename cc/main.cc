#include <stdio.h>
#include <stdlib.h>
#include <typeinfo>
#include "IntHolder.h"

int main(int argc, char** argv) {
  if (argc < 2) {
    return 1;
  }
  IntHolder* ih = new IntHolder(atoi(argv[1]));
  printf("name: %s\n", typeid(*ih).name() );
  ih->add(5);
  printf("%ld\n", ih->value());
  ih->sub(2);
  printf("%ld\n", ih->value());
  delete ih;
  return 0;
}
