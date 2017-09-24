#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread;

fn main() {
    env_logger::try_init().unwrap();




    {
        static _LOC: ::LogLocation =
            ::LogLocation{__line: 9u32,
                          __file: "src\\main.rs",
                          __module_path: "env_logger_test",};
        let lvl = ::LogLevel::Info;
        if lvl <= ::__static_max_level() && lvl <= ::max_log_level() {
            ::__log(lvl, "env_logger_test", &_LOC,
                    ::std::fmt::Arguments::new_v1(&["starting"],
                                                  &match () { () => [], }))
        }
    };
    let handles: Vec<_> =
        (0..10).map(|i|
                        {
                            thread::spawn(move ||
                                              {
                                                  {
                                                      static _LOC:
                                                             ::LogLocation =
                                                          ::LogLocation{__line:
                                                                            13u32,
                                                                        __file:
                                                                            "src\\main.rs",
                                                                        __module_path:
                                                                            "env_logger_test",};
                                                      let lvl =
                                                          ::LogLevel::Error;
                                                      if lvl <=
                                                             ::__static_max_level()
                                                             &&
                                                             lvl <=
                                                                 ::max_log_level()
                                                         {
                                                          ::__log(lvl,
                                                                  "env_logger_test",
                                                                  &_LOC,
                                                                  ::std::fmt::Arguments::new_v1(&["error from "],
                                                                                                &match (&i,)
                                                                                                     {
                                                                                                     (__arg0,)
                                                                                                     =>
                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                  ::std::fmt::Display::fmt)],
                                                                                                 }))
                                                      }
                                                  };
                                              })
                        }).collect();
    for handle in handles { handle.join().unwrap(); }
    ::io::_print(::std::fmt::Arguments::new_v1(&["Done\n"],
                                               &match () { () => [], }));
}
