pub struct Book{
  pub title: String,
  pub author: String,
}

pub fn add_book(title: &str, author: &str){
  let new_book= Book{
    title: title.to_string(),
    author: author.to_string(),
  };
  println!("Added book: {} by {}", new_book.title, new_book.author);
}
