const ffi = require('ffi');

const rustLib = ffi.Library('librust', {
  add: ['uint32', ['uint32', 'uint32']],
});

console.log('add: ' + rustLib.add(1, 2));