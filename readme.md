# Intro
Treadmilldb is a light-weight document database with master/master replication.
It is inspired by couchdb and riak and built on rust, leveldb, and zeromq.

# API
## Documents
GET http://server:1444/bucket/id (read)
PUT http://server:1444/bucket/id (update)
POST http://server:1444/bucket (create)

## Query
POST http://server:1444/bucket/_query
data: { map: "function(doc){ return doc.name}"}
result: { count: 1,
          rows: [{name: "Sam"}]}

## Federation
GET http://server:1444/_system/peers
result: { count: 1,
          peers: [{zmq_path: "tcp://otherbox.local:1444",
                   sequence_number: 34}]}
POST http://server:1444/_system/peers
data: { zmq_path: "xxx://123"}

