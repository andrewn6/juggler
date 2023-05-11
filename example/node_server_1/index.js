const express = require('express')
const app = express()
const port = 8001

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example service listening on port ${port}`)
})
