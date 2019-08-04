const ffi = require('ffi');

const rustLib = ffi.Library('librust', {
  add: ['int32', ['int32', 'int32']],
});

console.log('add: ' + rustLib.add(1, 2));