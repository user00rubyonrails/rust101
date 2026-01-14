use std::env;
use dotenv::dotenv;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    unsafe { env::set_var("RUST_LOG", "actix_web=info") };
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL mu\
    st be set");
    let app = blog_actix::Blog::new(8998);
    app.run(database_url)
}
