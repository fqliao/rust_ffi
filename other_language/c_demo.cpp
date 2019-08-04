#include "c_demo.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>
#include <iostream>

using namespace std;

int main(int argc, const char* argv[])
{
    cout << "add: " << add(1, 2) << endl;

    char* ptr_str = hello("World");
    cout << "\\nhello: " << ptr_str << endl;
    hello_free(ptr_str);

    Person* person = get_person("Alice", 12);
    cout << "\\nget_person: {name=\"" << person_get_name(person) << "\", age=" << person_get_age(person) << "}" << endl;
    person_free(person);

    return 0;
}