const ffi = require('ffi');

const rustLib = ffi.Library('librust', {
  add: ['int32', ['int32', 'int32']],
  hello: ['char *', ['string']],
  hello_free:['void', ['char *']]
});

console.log('add: ' + rustLib.add(1, 2));

function js_hello(str) {
  const str_ptr = rustLib.hello(str);
  try {
    return str_ptr.readCString();
  } finally {
    rustLib.hello_free(str_ptr);
  }
}
console.log('\\nhello: ' + js_hello('World'));