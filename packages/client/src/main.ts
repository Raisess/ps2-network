import { PS2NetworkClient } from "./ps2n-client";
import { UI, UIItem } from "./ui";

const IP_MASK = "192.168.{0}.{1}";
const SERVER_IP = PS2NetworkClient.getServerIp(
  PS2NetworkClient.ipFromMask(IP_MASK, "1", "10")
);
const SERVER_PORT = 8080;

console.log("INITING NET...");
IOP.loadDefaultModule(IOP.network);
Network.init();
console.log("DONE INITING NET!");

const font = new Font("default");
font.scale = 0.7;
const pad = Pads.get();
pad.setEventHandler();

const client = new PS2NetworkClient(SERVER_IP, SERVER_PORT);
const ui = new UI(font, pad);

const backToMainItem = new UIItem("Back", "- Back to previous menu", (ctx) => {
  ctx.ui.set("main");
});

// -------- MAIN SCREEN -------- //
const main = ui.createComponent("main", 'PS2 Network');

// -------- SEARCH SCREEN -------- //
main.addItem(new UIItem("Search Games", "- Search for games on the network", (ctx) => {
  const to = ctx.ui.createKeyboardComponent();
  to.setup("main", (_value, ctx) => {
    console.log(ctx.buffer.length, ctx.buffer);
    // -------- SEARCH RESULTS SUBSCREEN -------- //
    const to = ctx.ui.createComponent("results", "Results");
    to.addItem(backToMainItem);
    const results = client.search(encodeURIComponent(ctx.buffer))
    results.forEach((item) => {
      const size = item.size / 1_000_000_000;
      to.addItem(new UIItem(item.name, `- Size: ${size.toFixed(2)}GB`, (ctx) => {
        client.download(item.id);
        ctx.ui.set("main");
      }));
    });
    ctx.buffer = "";
  });
}));

// -------- DOWNLOADS SCREEN -------- //
main.addItem(new UIItem("My Downloads", "- List all running downloads", (ctx) => {
  const to = ctx.ui.createComponent("downloads", "Downloads");
  to.addItem(backToMainItem);

  const downloads = client.listDownloads();
  downloads.forEach((download) => {
    const size = download.size / 1_000_000_000;
    to.addItem(new UIItem(download.name, `- Size: ${size.toFixed(2)}GB | Status: ${download.status}`));
  });
}));

// -------- PING SCREEN -------- //
main.addItem(new UIItem("Ping Server", "- Sends a ping message to test the server", (ctx) => {
  const pong = client.ping();
  const to = ctx.ui.createComponent("ping", `Server ping: ${client.url} => ${pong}`);

  // -------- ADDRESS LOOKUP SUBSCREEN -------- //
  to.addItem(new UIItem("Address Lookup", "- Search for the server on your network", (ctx) => {
    const to = ctx.ui.createComponent("address_lookup", "@TODO: Looking up for address please wait...");
    // @TODO: lookup
    to.addItem(backToMainItem);
  }));
  to.addItem(backToMainItem);
}));

main.addItem(new UIItem("Exit", "- Exit to mc0:/BOOT/BOOT.ELF", () => {
  System.loadELF("mc0:/BOOT/BOOT.ELF");
}));

const canvas = Screen.getMode();

ui.set(main);
os.setInterval(() => {
  Screen.clear();
  ui.render();
  font.print(0, canvas.height - 40, "Made by: Danilo <github.com/Raisess>");
  Screen.flip();
}, 0);
