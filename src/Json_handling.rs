/*
 *   Copyright (c) 2020 Johannes Thorén
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

extern crate json;
extern crate reqwest;

// gets the json data from a subbreddit if you have provided a url and returns it as a JsonValue
pub fn get_json_data(URL:&str) -> json::JsonValue{
    let data = reqwest::get(&format!("{}", URL)).expect("could not load Json data").text().expect("could not get text");
    json::parse(data.as_str()).unwrap()
}


pub struct GetData {
    pub post_data_url: String,
    pub post_title: String,
    pub post_url: String,
}

pub fn get_post_data (json_data:json::JsonValue, child_index: usize) -> GetData{


    let post_data_url = format!("{}", json_data["data"]["children"][child_index]["data"]["url"]);
    let post_title = format!("{}", json_data["data"]["children"][child_index]["data"]["title"]);
    let post_url = format!("https://reddit.com{}", json_data["data"]["children"][child_index]["data"]["permalink"]);

    GetData {
        post_data_url: post_data_url,
        post_title: post_title,
        post_url: post_url,
    }

}



