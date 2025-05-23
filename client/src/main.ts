import { PS2NetworkClient } from "./ps2n-client";
import { UI, UIItem } from "./ui";

// @TODO: read this from config.txt file
const SERVER_IP = "192.168.3.10";
const SERVER_PORT = 8080;

console.log("INITING NET...");
IOP.loadDefaultModule(IOP.network);
Network.init();
console.log("DONE INITING NET!");

const font = new Font("default");
font.scale = 0.7;
const pad = Pads.get();
pad.setEventHandler();

const client = new PS2NetworkClient(SERVER_IP, SERVER_PORT)
const ui = new UI(font, pad);

const backToMainItem = new UIItem("Back", "- Back to previous menu", (ctx) => {
  ctx.parent.set("main");
});

// -------- MAIN SCREEN -------- //
const main = ui.createComponent("main", `PS2 Network: ${SERVER_IP}:${SERVER_PORT}`);

// -------- SEARCH SCREEN -------- //
main.addItem(new UIItem("Search Games", "- Search for games on the network", (ctx) => {
  const to = ctx.parent.createKeyboardComponent();
  to.setup("main", (_value, ctx) => {
    console.log(ctx.buffer.length, ctx.buffer);
    // -------- SEARCH RESULTS SUBSCREEN -------- //
    const to = ctx.parent.createComponent("results", "Results");
    to.addItem(backToMainItem);
    const results = client.search(encodeURIComponent(ctx.buffer))
    results.forEach((item) => {
      to.addItem(new UIItem(item.name, "TODO: add item description", (ctx) => {
        client.download(item.id);
        ctx.parent.set("main");
      }));
    });
    ctx.buffer = "";
  });
}));

// -------- DOWNLOADS SCREEN -------- //
main.addItem(new UIItem("My Downloads", "- List all running downloads", (ctx) => {
  const to = ctx.parent.createComponent("downloads", "Downloads");
  to.addItem(backToMainItem);

  const downloads = client.listDownloads();
  downloads.forEach((download) => {
    to.addItem(new UIItem(download.name, "TODO: add download state"));
  });
}));

// -------- PING SCREEN -------- //
main.addItem(new UIItem("Ping Server", "- Sends a ping message to test the server", (ctx) => {
  const pong = client.ping();
  const to = ctx.parent.createComponent("ping", `Server ping: ${pong}`);
  to.addItem(backToMainItem);
}));

main.addItem(new UIItem("Exit", "- Exit to mc0:/BOOT/BOOT.ELF", () => {
  System.loadELF("mc0:/BOOT/BOOT.ELF");
}));

const canvas = Screen.getMode();

ui.set("main");
os.setInterval(() => {
  Screen.clear();
  ui.render();
  font.print(0, canvas.height - 40, "Made by: Danilo <github.com/Raisess>");
  Screen.flip();
}, 0);
