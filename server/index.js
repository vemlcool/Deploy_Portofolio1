const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')
const bodyParser = require('body-parser')

app.use(cors())
app.use(bodyParser.json()) // for parsing application/json
app.use(bodyParser.urlencoded({ extended: true })) // for parsing application/x-www-form-urlencoded



app.post('/NewConnector', (req, res) => {

  console.log(req.body)
  var data
  res.status(200).send(data)
})

app.get('/NewConnector', (req, res) => {

  console.log(req.body)
  var data
  res.status(200).send(data)
})


app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})