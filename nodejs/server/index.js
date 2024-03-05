const express = require("express");
const multer = require("multer");
const rimage = require('rimage');
const fs = require("fs");
const path = require("path");
const app = express();
const port = process.env.PORT || 3000;

const storage = multer.diskStorage({
  destination: function (req, file, cb) {
    cb(null, '/tmp');
  },
  filename: function (req, file, cb) {
    cb(null, "upload-" + Date.now() + "-" + file.originalname);
  }
});

const upload = multer({ storage: storage });

app.set("view engine", "ejs");

app.get("/", (req, res) => {
  res.sendFile(__dirname + "/views/index.html");
});

app.post("/upload", upload.single("file"), (req, res) => {
  const output_path = rimage.png_to_jpg(req.file.path);
  const image = fs.readFileSync(output_path);
  const base64Data = image.toString("base64");
  res.render("image.html.ejs", { image: base64Data, format: "jpg" });
});

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`);
});
