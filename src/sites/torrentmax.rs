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
    let host_url = consts::TORRENTMAX_HOST_URL;
    let url = format!("{}/bbs/search.php?search.php&stx={}", host_url, keyword);
    let doc = common::get_doc(&url).unwrap();
    for node in doc.find(Class("media-heading")) {
        let title = node.text().trim().to_owned();
        let link = node.find(Name("a"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let link = link.to_owned();
        result.push((title, link));
    }
    Ok(result)
}

fn fetch_magnet(url: &str) -> Result<String, Box<dyn Error>>{
    let doc = common::get_doc(&url).unwrap();
    let mut magnet = String::from("no magnet");
    for node in doc.find(Class("list-group")) {
        let temp = node.text();
        if temp.contains("magnet:?xt=urn:btih:") {
            magnet = temp.trim().to_owned();
        }
    }
    Ok(magnet)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fetch_magnet_func() {
        let doc = Document::from(include_str!("../test_data/torrentmax_bbs.html"));
        let mut magnet = String::from("no magnet");
        for node in doc.find(Class("list-group")) {
            let temp = node.text();
            if temp.contains("magnet:?xt=urn:btih:") {
                magnet = temp.trim().to_owned();
            }
        }
       assert_eq!(
           &magnet[..],
           "magnet:?xt=urn:btih:1ad4272fc2dec93dfdb9a8d514881193e749993b",
       )
    }
    #[test]
    fn test_fetch_search_data_func() {
        let mut result = vec![];
        let doc = Document::from(include_str!("../test_data/torrentmax_search.html"));
        for node in doc.find(Class("media-heading")) {
            let title = node.text().trim().to_owned();
            let link = node.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
            let link = link.to_owned();
            result.push((title, link));
        }
        assert_eq!(
            ("동상이몽2 너는 내운명.E184.210215.720p-NEXT", 
            "https://torrentmax15.com/max/VARIETY/23823"),
            (&result[0].0[..], &result[0].1[..]),
        );
    }
}