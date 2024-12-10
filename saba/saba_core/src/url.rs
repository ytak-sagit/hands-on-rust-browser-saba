use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}
impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    fn is_http(&self) -> bool {
        self.url.starts_with("http://")
    }

    fn extract_host(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        // ポート番号を除く
        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            // HTTPのデフォルトのポート番号を返却
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart = url_parts[1].splitn(2, '?').collect::<Vec<&str>>();
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart = url_parts[1].splitn(2, '?').collect::<Vec<&str>>();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_url_host() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn should_parse_url_host_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn should_parse_url_host_port_path() {
        let url = "http://example.com:8080/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn should_parse_url_host_path() {
        let url = "http://example.com/test.json".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "test.json".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn should_parse_url_host_port_path_searchpart() {
        let url = "http://example.com:18080/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "18080".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }
}
