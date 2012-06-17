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
result -> { count: 1,
          rows: [{name: "Sam"}]}
```

## Activity Log
```
GET http://server:1444/bucket/_activity
```

## Federation
```
GET http://server:1444/_system/peers
result -> { count: 1,
            peers: [{zmq_path: "tcp://otherbox.local:1444",
                     sequence_number: 34}]}
POST http://server:1444/_system/peers
data -> { name: "box1",
          zmq_path: "xxx://123"}
DELETE http://server:1444/_system/peers/box1
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

## Merge conflicts
