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
activity_log: []
* bucket1: sequence: 0,
           rows: []

box2.local
activity_log: []
* bucket1: sequence: 0,
           rows: []
```

Create record.
```
$ curl -X POST -d {id:"document1", color:"blue"} http://box1.local:1444/bucket1/
{status: "OK", id:"document1", rev:"rev1-223abc"}
```

Record created.
```
box1.local
activity_log: [{seq: 1, id:"document1", changes["rev1-223abc"]}]
* bucket1: sequence: 1,
           rows: [{id:"document1", rev:"123abc", color:"blue"}]

box2.local
activity_log: []
* bucket1: sequence: 0,
           rows:[]
```

Broadcast new activity log entry
```
box1.local
zmq_send(peers, {seq: 1, id:"document1", changes: ["rev1-223abc"]})

box2.local
recivied_entry = zmq_recv(peers)
```

What happens next on box2 depends on the state of its bucket1.
The following is pseudo-code to show the steps.

```
if received_entry.seq == bucket1.sequence+1
  if received_entry.deleted == true # DELETE
    delete_document_and_index_data(received_entry.id)
  end
  new_document = fetch(received_entry.host,
                       received_entry.id, received_entry.rev)
  if bucket1.contains(received_entry.id)
    update_document_and_index_data(new_document) # UPDATE
  else
    create_document_and_index_data(new_document) # CREATE
  end
  bucket1.sequence = received_entry.seq
else
  # ignore out-of-sequence entry
end
```

The delete and create methods are straight-forward. The update method
can end up with merges and merge conflicts. Also pretend the received_entry
is the entire document at the given revision, which is passed to the update
function.

```
def update_document_and_index_data(new_document)
  existing_document = bucket1.get(new_document.id)
  if new_document.rev_num > existing_document.rev_num
    # longest history wins, shorter history is lost
    winning_document = new_document
  end
  if new_document.rev_num == existing_document.rev_num
    # random winner
    if new_document.hash > existing_document.hash
      winning_document = new_document
    else
      winning_document = existing_document
    end
  end
  # longest history wins, shorter history is lost!
  existing_document.rev = winning_document.rev
  existing_document.body = winning_document.body
end
```
