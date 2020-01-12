'use strict'

const {strictEqual} = require('assert')
const {hash, createHash} = require('.')

const input = Buffer.from('some input!', 'utf8')
const expected = '948abea72a9c6bd221d734457c15def1be448efef2d48b91882e73cd9254f0bb'

strictEqual(hash(input), expected)

const h = createHash()
h.update(input.slice(0, 4))
h.update(input.slice(4))
strictEqual(h.digest(), expected)

console.info('looks good!')
