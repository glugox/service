use service;
use service::discovery::send_registration;
use futures::executor::block_on;

fn main() {
    service::load_env(None);
    service::rocket().launch();

}