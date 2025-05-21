class GameData {
  constructor(public id: string, public name: string) {}
}

export class PS2NetworkClient {
  private readonly request: Request;
  private readonly url: string;

  constructor(host: string, port: number) {
    this.request = new Request();
    this.url = `http://${host}:${port}`;
  }

  public ping(): string {
    try {
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
      return JSON.parse(response.text).map((item: any) => new GameData(
        item.id,
        item.name,
      ));
    } catch {
      console.log("[PS2NetworkClient]: Failed to list downloads");
      return [];
    }
  }

  public search(key: string): GameData[] {
    try {
      const response = this.request.get(`${this.url}/search?key=${key}`);
      return JSON.parse(response.text).map((item: any) => new GameData(
        item.id,
        item.name,
      ));
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
}
