// treadmilldb
use epoll;
use socket;

type config = {port: u16};

fn main(args: [str]) {
  // listen to http
  let config = config();
  let epoll_fd = setup(config);

  io::println("treadmill db has started. time to get moving.")
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
          io::println("Listening on "+#fmt("%u", config.port as uint));
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

  ret ep;
}