use crate::*;

pub fn print_table(p: HashMap<String, String>) {
    let mut table = Table::new();
    table.set_titles(row!["Title", "Magnet"]);
    for (k, v) in &p {
        table.add_row(Row::new(vec![
             Cell::new(k),
             Cell::new(v),
        ]));
      }
      table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
      table.printstd();
}

pub fn get_doc(url: &str) -> Result<Document, Box<dyn Error>>  {
  let client = Client::new();
  let resp = client.get(&url[..])
      .header(USER_AGENT, consts::MY_USER_AGENT)
      .send()?;
  let doc = Document::from_read(resp)?;
  Ok(doc)
}

pub fn execute(keyword: String) -> HashMap<String, String>{
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();
  let tx2 = tx.clone();
  let keyword = Arc::new(keyword);
  let k1 = Arc::clone(&keyword);
  let k2 = Arc::clone(&keyword);
  thread::spawn(move || {
      let data = ttobogo::crawl(&k1);
      tx.send(data).unwrap();
  });
  thread::spawn(move || {
      let data = torrentsir::crawl(&keyword);
      tx1.send(data).unwrap();
  });
  thread::spawn(move || {
    let data = torrentmax::crawl(&k2);
    tx2.send(data).unwrap();
  });
  let mut data = HashMap::new();
  for r in rx {
      for i in r {
          data.insert(i.0, i.1);
      }
  }
  data
}