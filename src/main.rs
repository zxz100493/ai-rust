fn main() {
    let s = get_access_token();
    println!("str is : {}", s);
}

fn get_access_token() -> String {
    let url = "https://aip.baidubce.com/oauth/2.0/token";
    let api_key = match std::env::var("API_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("not found api_key");
            return String::from("");
        }
    };
    
    let api_secret = match std::env::var("api_secret") {
        Ok(val) => val,
        Err(_) => {
            println!("not found api_key");
            return String::from("");
        }
    };



    let post_data = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}",
        api_key, api_secret
    );



    String::from("test")
}

/* func GetAccessToken() string {
	url := "https://aip.baidubce.com/oauth/2.0/token"
	postData := fmt.Sprintf("grant_type=client_credentials&client_id=%s&client_secret=%s", api_key, api_secret)
	resp, err := http.Post(url, "application/x-www-form-urlencoded", strings.NewReader(postData))
	if err != nil {
		fmt.Println(err)
		return ""
	}
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return ""
	}
	accessTokenObj := map[string]string{}
	json.Unmarshal([]byte(body), &accessTokenObj)
	return accessTokenObj["access_token"]
} */