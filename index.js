'use strict'

const {HashCtx, hash} = require('./native/index.node')

const createHash = () => new HashCtx()

module.exports = {
	createHash,
	hash
}
