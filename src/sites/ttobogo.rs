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
    let host_url = consts::TTOBOGO_HOST_URL;
    let url = format!("{}/search?skeyword={}", host_url, keyword);
    let doc = common::get_doc(&url).unwrap();
    for node in doc.find(Class("subject")) {
        let title = node.attr("title").unwrap();
        let link = node.attr("href").unwrap();
        result.push((title.to_owned(), link.to_owned()));
    }
    Ok(result)
}
    
fn fetch_magnet(url: &str) -> Result<String, Box<dyn Error>> {
    let doc = common::get_doc(&url).unwrap();
    let mut magnet = "no magnet";
    for node in doc.find(Name("td")) {
        for child in node.children() {
            if let Some(x) = child.attr("onclick") {
                if x.contains("magnet:?xt=urn:btih:") {
                    let v: Vec<&str> = x.split('\'').collect();
                    magnet = v[1];
                }
            }
        }
    }
    Ok(magnet.to_owned())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fetch_magnet_func() {
        let doc = Document::from(include_str!("../test_data/ttobogo_bbs.html"));
        let mut magnet = "no magnet";
        for node in doc.find(Name("td")) {
            for child in node.children() {
                if let Some(x) = child.attr("onclick") {
                    if x.contains("magnet:?xt=urn:btih:") {
                        magnet = x;
                    }
                }
            }
        }
        let v: Vec<&str> = magnet.split('\'').collect();
        assert_eq!(
            v[1],
            "magnet:?xt=urn:btih:00a2fc4594123502841e565e24b328dbb9c0cb83",
        );
    }
    #[test]
    fn test_fetch_search_data_func() {
        let mut result = vec![];
        let doc = Document::from(include_str!("../test_data/ttobogo_search.html"));
        for node in doc.find(Class("subject")) {
            let title = node.attr("title").unwrap();
            let link = node.attr("href").unwrap();
            result.push((title, link));
        }
        assert_eq!(
            ("시지프스 E02.210218.1080p.WEB-DL.x264.AAC-Deresisi", 
            "https://www1.ttobogo.net/post/204067"),
            result[0],
        )
    }
}