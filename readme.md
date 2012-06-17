# Intro
Treadmilldb is a light-weight document database with master/master replication.
It is inspired by couchdb and riak and built on rust, leveldb, and zeromq.

# API
## Documents
```
GET http://server:1444/bucket/id (read)
PUT http://server:1444/bucket/id (update)
POST http://server:1444/bucket (create)
```

## Query
```
POST http://server:1444/bucket/_query/name
data -> { map: "function(doc){ return doc.name}"}
GET http://server:1444/bucket/_query/name
result -> { count: 1,
            rows: [{name: "Sam"}]}
```

## Activity Feed
```
GET http://server:1444/bucket/_activity
```

## Federation
```
GET http://server:1444/_system/peers
result -> { count: 1,
            peers: [{name: "box1.local",
                     zmq_uri: "tcp://box1.local:1444",
                     sequence_number: 34}]}
POST http://server:1444/_system/peers
data -> { zmq_uri: "xxx://123"}
DELETE http://server:1444/_system/peers/box1.local
```

# Implementation
## View indexes
For each query name, a list of answers is established.
{key: "document_id", value: "map function return value"}
To answer a query, return the key/value pairs that match the
query term.

When a document is created, append that key/value to the view
index. If a document is modified, recompute the value for that
document. If a document is removed, remove the key/value.

## Activity feed
Every change to a document creates an activity entry with
the details of that change. The activities are numbered
sequentially with a sequence number.

```
{seq: 1, id: "abc123", changes: ["rev1-abc1"]}
{seq: 2, id: "abc123", changes: ["rev2-zyb3"], deleted: true}
```

Both clients and peers can use the activity feed to keep their
local data fresh.

## Sync/Merge

case: Record creation

Initial state
```
box1.local

box2.local
sequence: 0
activity_log: []
```

Create record.
```
$ curl -X POST -d {id:"document1", color:"blue"} http://box1.local/bucket1/
```

Record created.
```
box1.local
bucket1: [{id:"document1", rev:"123abc", color:"blue"}]
sequence: 1
activity_log: [{seq: 1, id:"document1", changes["123abc"]}

box2.local
sequence: 0
activity_log: []
```
