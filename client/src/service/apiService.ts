import axios from "axios";

const url = "http://" + window.location.hostname + ":8080/";

export const get = (path: string) => {
  // console.log("GET", path);
  return axios.get(url + path).then((data) => data.data);
};

export const post = (path: string, body: Object) => {
  console.log("POST", path, body);
  return axios
    .post(url + path, body)
    .then((response) => response.data)
    .catch(() => "Post failed");
};
