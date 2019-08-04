const ffi = require('ffi');

const rustLib = ffi.Library('librust', {
  add: ['int32', ['int32', 'int32']],
  hello: ['char *', ['string']],
  hello_free:['void', ['char *']],
  get_person: ['pointer', ['string', 'uint32']],
  person_get_name: ['char *', ['pointer']],
  person_get_age: ['uint32', ['pointer']],
  person_free: ['void', ['pointer']]
});

console.log('add: ' + rustLib.add(1, 2));

function hello(str) {
  const strPtr = rustLib.hello(str);
  try {
    return strPtr.readCString();
  } finally {
    rustLib.hello_free(strPtr);
  }
}
console.log('\\nhello: ' + hello('World'));


person = rustLib.get_person('Alice', 12);
try {
  name = rustLib.person_get_name(person).readCString();
  age = rustLib.person_get_age(person);
  console.log('\\nget_person: {name=\"' + name + '\", age=' + age +'}');
} finally {
  rustLib.person_free(person);
}
