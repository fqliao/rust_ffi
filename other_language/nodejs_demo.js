const ffi = require('ffi');

const rustLib = ffi.Library('librust', {
  add: ['int32', ['int32', 'int32']],
  count_chars: ['uint32', ['string']]
});

console.log('add: ' + rustLib.add(1, 2));
console.log();
console.log('\\ncount_chars: ' + rustLib.count_chars('Hello, World!'))