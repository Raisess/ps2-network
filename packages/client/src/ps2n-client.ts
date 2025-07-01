class GameData {
  constructor(public id: string, public name: string, public status: string, public size: number) {}
}

export class PS2NetworkClient {
  public static getServerIp(defaultIp: string): string {
    const exists = std.exists("ip.txt");
    if (!exists) return defaultIp

    const file = std.open("ip.txt", "r");
    const ip = file.readAsString().trim();
    return ip;
  }

  public static ipFromMask(mask: string, a: string, b: string): string {
    return mask.replace("{0}", a).replace("{1}", b);
  }

  private readonly request: Request;

  constructor(private host: string, private port: number) {
    this.request = new Request();
  }

  public get url(): string {
    return `http://${this.host}:${this.port}`
  }

  public ping(host?: string): string {
    try {
      if (host) this.host = host

      const response = this.request.get(`${this.url}/ping`);
      return response.text;
    } catch {
      console.log("[PS2NetworkClient]: Failed to ping server");
      return 'Failed!';
    }
  }

  public listDownloads(): GameData[] {
    try {
      const response = this.request.get(`${this.url}/downloads`);
      return this.format(response.text);
    } catch {
      console.log("[PS2NetworkClient]: Failed to list downloads");
      return [];
    }
  }

  public search(key: string): GameData[] {
    try {
      const response = this.request.get(`${this.url}/search?key=${key}`);
      return this.format(response.text);
    } catch {
      console.log("[PS2NetworkClient]: Failed to search");
      return [];
    }
  }

  public download(id: string): void {
    try {
      this.request.get(`${this.url}/download?id=${id}`);
    } catch {
      console.log("[PS2NetworkClient]: Failed to download");
    }
  }

  private format(text: string): GameData[] {
    return JSON.parse(text).map((item: any) => new GameData(
      item.id,
      item.name,
      item.status,
      item.size,
    ));
  }
}
