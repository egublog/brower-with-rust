use alloc::string::String;
use alloc::string::ToString;

/// URLを表現する構造体です。
/// HTTP URLのパース処理を提供します。
#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    searchpart: String,
}

impl Url {
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

    /// URLがHTTPスキームかどうかを確認します。
    ///
    /// # 戻り値
    /// * `true` - URLがhttp://で始まる場合
    /// * `false` - それ以外の場合
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    /// URLからホスト名を抽出します。
    ///
    /// # 戻り値
    /// * `String` - 抽出されたホスト名
    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    /// URLからパスを抽出します。
    ///
    /// # 戻り値
    /// * `String` - 抽出されたパス。パスが存在しない場合は空文字列
    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string() // (d3)
    }

    /// URLからポート番号を抽出します。
    ///
    /// # 戻り値
    /// * `String` - 抽出されたポート番号。明示的なポート指定がない場合は"80"
    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    /// URLからクエリパラメータ部分を抽出します。
    ///
    /// # 戻り値
    /// * `String` - 抽出されたクエリパラメータ。クエリパラメータが存在しない場合は空文字列
    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }

    /// URLを解析し、各コンポーネント（ホスト、ポート、パス、クエリパラメータ）に分解します。
    ///
    /// # 戻り値
    /// * `Ok(Self)` - 解析が成功した場合
    /// * `Err(String)` - 解析に失敗した場合（HTTPスキーム以外）
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
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
    fn test_url_host_port() {
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
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "/index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse());
    }
}
