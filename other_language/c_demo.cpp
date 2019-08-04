#include <iostream>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>
#include "c_demo.h"

using namespace std;

int main(int argc, const char* argv[]) {

  cout << "add: " << add(1, 2) << endl;
  cout << "\\ncount_chars: " << count_chars("Hello, World!") << endl;

  return 0;
}