use crate::*;

pub fn crawl(keyword: &str) -> Vec<(String, String)> {
    let result = fetch_search_data(keyword).unwrap();
    let m = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    for r in result {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let title = r.0;
            let magnet = fetch_magnet(&r.1[..]).unwrap();
            let mut m = m.lock().unwrap();
            (*m).push((title, magnet));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let data = &*m.lock().unwrap();
    data.to_vec()
}

fn fetch_search_data(keyword: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut result = vec![];
    let host_url = consts::TORRENTQQ_HOST_URL;
    let url = format!("{}/search?q={}", host_url, keyword);
    let doc = common::get_doc(&url).unwrap();
    for node in doc.find(Class("subject")) {
        let title = node.attr("title").unwrap().to_owned();
        let link = node.attr("href").unwrap().to_owned();
        result.push((title, link));
     }
     Ok(result)
}
    
fn fetch_magnet(url: &str) -> Result<String, Box<dyn Error>>{
    let magnet_prefix = "magnet:?xt=urn:btih:";
    let doc = common::get_doc(&url).unwrap();
    let m = doc.find(Name("td"))
        .next()
        .unwrap()
        .find(Name("li"))
        .next()
        .unwrap()
        .text();
    let magnet = format!(
         "{}{}",
         magnet_prefix, 
         m.strip_prefix("Info Hash: ").unwrap(),
    );
    Ok(magnet.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fetch_search_data_func() {
        let mut result = vec![];
        let doc = Document::from(include_str!("../test_data/torrentqq_search.html"));
        for node in doc.find(Class("subject")) {
           let title = node.attr("title").unwrap();
           let link = node.attr("href").unwrap();
           result.push((title, link));
        }
        assert_eq!(
            ("동상이몽2 너는 내운명.E184.210215.720p-NEXT", 
            "https://torrentqq75.com/torrent/med/403613.html"),
            result[0]
        )
    }
    #[test]
    fn test_fetch_magnet_func() {
        let doc = Document::from(include_str!("../test_data/torrentqq_bbs.html"));
       let m = doc.find(Name("td"))
            .next()
            .unwrap()
            .find(Name("li"))
            .next()
            .unwrap()
            .text();
        let magnet = format!(
            "{}{}",
            "magnet:?xt=urn:btih:", 
            m.strip_prefix("Info Hash: ").unwrap(),
        );
       assert_eq!(
           "magnet:?xt=urn:btih:1ad4272fc2dec93dfdb9a8d514881193e749993b",
           magnet,
       )
    }
}