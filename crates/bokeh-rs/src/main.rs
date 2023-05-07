use polars::lazy::prelude::*;
use polars::prelude::*;

struct Resources;
struct Document;

fn main() {
  let df = df! [
    "A"        => [1, 2, 3, 4, 5],
    "fruits"   => ["banana", "banana", "apple", "apple", "banana"],
    "B"        => [5, 4, 3, 2, 1],
    "cars"     => ["beetle", "audi", "beetle", "beetle", "beetle"],
    "optional" => [Some(28), Some(300), None, Some(2), Some(-30)],
  ].unwrap();

  df.lazy().select([col("A")]);

  println!("Bokeh");
}
