import ky from "ky";

const api = ky.create();

export const generate_urlet = (url) => {
  return api.post("/", {body: url}).json();
};