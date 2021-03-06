// treadmilldb
use std;
use epoll;
use socket;

fn main(_args: [str]) {
  // configuration
  let config = setup();
  let (epoll_fd, socket) = result::get(config);

  io::println("treadmill db has started. time to get moving.");
  loop {
    listen(epoll_fd, socket);
  }
}

fn setup() -> result::result<(int, @socket::socket_handle),str> {
  let ep = epoll::epoll_create();

  alt socket::bind_socket("localhost", 1444) {
    result::ok(socket) {
      socket::listen(socket, 1);
      let ok = epoll::epoll_ctl(ep, epoll::EPOLL_CTL_ADD, **socket as int,
                       {events: epoll::EPOLLIN, data:**socket as u64});
      if ok == 0 {
        io::println("Listening on :"+#fmt("%u", 1444 as uint));
        ret result::ok((ep, socket));
      } }
    result::err(e) {
      io::println(#fmt("bind error: %s", e)); }
  }
  ret result::err("error");
}

fn listen(ep: int, sock: @socket::socket_handle)  {
  let out_events: [mut epoll::epoll_event] = [mut{events:0i32, data:0u64},
                                                 {events:0i32, data:0u64}];
  let retu = epoll::epoll_wait(ep, out_events, -1);

  let tuple = socket::accept(sock);
  let (s, msg) = result::get(tuple);
  io::println("pre spawn");
  task::spawn {||
    dispatch(@socket::socket_handle(s), msg);
  }
  io::println("post spawn\n");
}

fn dispatch(socket: @socket::socket_handle, host: str) {
  io::println(#fmt("%d", **socket as int) + "writing");
  str::as_bytes("Hello") {|buf|
    socket::send(socket, buf);
  }
  io::println(#fmt("%d", **socket as int) + "written");
}