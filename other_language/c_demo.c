#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern uint32_t
add(uint32_t, uint32_t);

int main(void) {
  uint32_t sum = add(1, 2);
  printf("add: %d", sum);
}