
const express = require('express')
const fs = require('fs')
const app = express()
const port = 3000

app.get('/', (req, res) => {
    let ret = fs.readFileSync('./contracts/nft/target/wasm32-unknown-unknown/release/non_fungible_token.wasm');
    console.log(ret);

    res.send('Hello World!');
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})