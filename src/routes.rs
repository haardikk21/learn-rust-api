use std::collections::HashMap;
use warp::http;

use crate::store;

pub async fn add_grocery_list_item(
  item: store::Item,
  store: store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  store.grocery_list.write().insert(item.name, item.quantity);

  return Ok(warp::reply::with_status(
    "Added item to grocery list",
    http::StatusCode::CREATED,
  ));
}

pub async fn get_grocery_list(store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  let mut result = HashMap::new();
  let r = store.grocery_list.read();

  for (k, v) in r.iter() {
    result.insert(k, v);
  }

  return Ok(warp::reply::json(&result));
}

pub async fn delete_grocery_list_item(
  id: store::Id,
  store: store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  store.grocery_list.write().remove(&id.name);

  return Ok(warp::reply::with_status(
    "Removed item from grocery list",
    http::StatusCode::OK,
  ));
}

pub async fn update_grocery_list(
  item: store::Item,
  store: store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  store.grocery_list.write().insert(item.name, item.quantity);

  return Ok(warp::reply::with_status(
    "Updated items in the grocery list",
    http::StatusCode::CREATED,
  ));
}
