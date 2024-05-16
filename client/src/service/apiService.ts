import axios from "axios";

const url = "http://" + window.location.hostname + ":8080/";

export const get = (path: string) => {
  // console.log("GET", path);
  return axios
    .get(url + path)
    .then((data) => data.data)
    .catch(() => {
      // console.log("Get failed");
      return undefined;
    });
};

export const post = (path: string, body: Object) => {
  console.log("POST", path, body);
  return axios
    .post(url + path, body)
    .then((response) => response.data)
    .catch(() => {
      // console.log("Post failed");
      return undefined;
    });
};
