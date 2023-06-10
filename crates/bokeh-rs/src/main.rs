/*
struct Resources;
struct Document;

trait Layoutable {
}

struct Row {
  children: Vec<Box<dyn Layoutable>>,
}
struct Column;
struct Grid;
struct Plot;
struct GridPlot;
*/

/*
fn main() {
  println!("Bokeh");
}
*/

use warp::Filter;

#[tokio::main]
async fn main() {
  //pretty_env_logger::init();

  // GET /hello/warp => 200 OK with body "Hello, warp!"
  let hello = warp::path!("hello" / String)
      .map(|name| format!("Hello, {}!\n", name));

  //let log = warp::log("bokeh-rs");
  let log = warp::any().map(|| { println!("foo"); warp::reply() });

  let routes = hello.or(log);

  warp::serve(routes)
    .run(([127, 0, 0, 1], 8000))
    .await;
}
