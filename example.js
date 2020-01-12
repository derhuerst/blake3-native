'use strict'

const {hash, createHash} = require('.')

const input = Buffer.from('some input!', 'utf8')

console.log('at once:', hash(input))

const h = createHash()
h.update(input.slice(0, 4))
h.update(input.slice(4))
console.log('chunked:', h.digest())
