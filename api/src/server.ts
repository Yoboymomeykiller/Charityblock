import express from "express";
import bodyParser from "body-parser";
import cors from "cors";
const app = express();
app.use(cors());
app.use(bodyParser.json());
app.get("/api/health", (_req, res) => {
  res.json({ status: "ok", cluster: process.env.SOLANA_CLUSTER || "devnet" });
});
const port = process.env.PORT || 8080;
app.listen(port, () => {
  console.log(`API running on port ${port}`);
});
