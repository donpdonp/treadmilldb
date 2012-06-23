// treadmilldb
use std;
use epoll;
use socket;

fn main(_args: [str]) {
  // configuration
  let ok = setup();

  io::println("treadmill db has started. time to get moving.");
  listen(result::get(ok));
}


fn setup() -> result::result<(int, @socket::socket_handle),str> {
  let ep = epoll::epoll_create();

  alt socket::bind_socket("localhost", 1444) {
    result::ok(s) {
      alt socket::listen(s, 1) {
        result::ok(socket) {
          io::println("socket fd:"+#fmt("%d", **socket as int));
          let ok = epoll::epoll_ctl(ep, epoll::EPOLL_CTL_ADD, **socket as int,
                           {events: epoll::EPOLLIN, data:**socket as u64});
          if ok == 0 {
            io::println("Listening on :"+#fmt("%u", 1444 as uint));
            ret result::ok((ep, s));
          }
        }
        result::err(e) {
          io::println(#fmt("listen error: %s", e));
        }
      }
    }
    result::err(e) {
      io::println(#fmt("bind error: %s", e));
    }
  }
  ret result::err("fuck");
}

fn listen(ep: (int, @socket::socket_handle))  {
  let (a,b) = ep;
  io::println("epfd:"+#fmt("%d", 1444));
  let out_events: [mut epoll::epoll_event] = [mut{events:0i32, data:0u64},
                                                 {events:0i32, data:0u64}];
  let retu = epoll::epoll_wait(a, out_events, -1);
  io::println("fish on "+#fmt("%d", retu ));
}