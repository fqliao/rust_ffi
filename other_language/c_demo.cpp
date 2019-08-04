#include <iostream>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>
#include "c_demo.h"

using namespace std;

int main(int argc, const char* argv[]) {
  int sum = add(1, 2);
  cout << "add: " << sum << endl;
  return 0;
}