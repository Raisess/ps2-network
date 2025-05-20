import { PS2NetworkClient } from "./ps2n-client";
import { UI, UIItem } from "./ui";

const SERVER_IP = "192.168.3.10";
const SERVER_PORT = 8080;

IOP.loadDefaultModule(IOP.network);
console.log("INITING...");
Network.init();
console.log("DONE!");

const font = new Font("default");
const pad = Pads.get();
pad.setEventHandler();

const client = new PS2NetworkClient(SERVER_IP, SERVER_PORT)
const ui = new UI(font, pad);

const backToMainItem = new UIItem("Back", (ctx) => {
  ctx.parent.set("main");
});

// -------- MAIN SCREEN -------- //
const main = ui.createComponent("main", `PS2 Network: ${SERVER_IP}:${SERVER_PORT}`);

// -------- SEARCH SCREEN -------- //
main.addItem(new UIItem("Search Games", (ctx) => {
  const to = ctx.parent.createComponent("search", "Search");
  to.reset();
  to.addItem(backToMainItem);
}));

// -------- DOWNLOADS SCREEN -------- //
main.addItem(new UIItem("My Downloads", (ctx) => {
  const to = ctx.parent.createComponent("downloads", "Downloads");
  to.reset();
  to.addItem(backToMainItem);

  const downloads = client.listDownloads();
  downloads.forEach((download) => {
    to.addItem(new UIItem(download.name));
  });
}));

ui.set("main");
os.setInterval(() => {
  Screen.clear();
  ui.render();
  Screen.flip();
}, 0);
