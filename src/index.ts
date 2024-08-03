import { readFile } from "node:fs";

readFile("./project/lines.txt", "utf-8", (error, data) => {
  if (error) {
    console.error(error);
  }

  console.log("yoo", data);
});
