#include <iostream>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>
#include "c_demo.h"

using namespace std;

int main(int argc, const char* argv[]) {

  cout << "add: " << add(1, 2) << endl;

  char *ptr_str = hello("World");
  cout << "\\nhello: " << ptr_str << endl;
  hello_free(ptr_str);

  return 0;
}