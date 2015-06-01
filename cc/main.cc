#include <stdio.h>
#include <stdlib.h>
#include "IntHolder.h"

int main(int argc, char** argv) {
  if (argc < 2) {
    return 1;
  }
  IntHolder* ih = new_IntHolder(atoi(argv[1]));
  ih->add(5);
  printf("%ld\n", ih->value());
  ih->sub(2);
  printf("%ld\n", ih->value());
  delete_IntHolder(ih);
  return 0;
}
