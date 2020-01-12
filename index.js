'use strict'

const {BLAKE3Hash, hashBLAKE3} = require('./native/index.node')

const createBLAKE3Hash = () => new BLAKE3Hash()

module.exports = {
	createHash: createBLAKE3Hash,
	hash: hashBLAKE3
}
