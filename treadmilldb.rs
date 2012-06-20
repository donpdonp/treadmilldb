// treadmilldb
use epoll;
use socket;

type config = {port: u16};

fn main(args: [str]) {
  // listen to http
  let config = config();
  let epoll_fd = setup(config);

  io::println("treadmill db has started. time to get moving.");
  listen(epoll_fd);
}

fn config() -> config {
  // load from file
  ret {port: 1444_u16};
}

fn setup(config: config) -> int {
  let ep = epoll::epoll_create();

  alt socket::bind_socket("localhost", config.port) {
    result::ok(s) {
      alt socket::listen(s, 1) {
        result::ok(socket) {
          io::println("socket fd:"+#fmt("%d", **socket));
          let ok = epoll::epoll_ctl(ep, epoll::EPOLL_CTL_ADD, **socket,
                           {events: epoll::EPOLLIN, data:**socket as u64});
          if ok == 0 {
            io::println("Listening on :"+#fmt("%u", config.port as uint));
            ret ep;
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
  ret -1; // error handling
}

fn listen(fd: int) {
  io::println("fd:"+#fmt("%d", fd));
  let out_events: [mut epoll::epoll_event] = [mut{events:0i32, data:0u64},
                                                 {events:0i32, data:0u64}];
  epoll::epoll_wait(fd, out_events, -1);
  io::println("fish on");
}