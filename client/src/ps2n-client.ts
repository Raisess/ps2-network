class GameData {
  name: string;
}

export class PS2NetworkClient {
  private readonly request: Request;
  private readonly url: string;

  constructor(host: string, port: number) {
    this.request = new Request();
    this.url = `http://${host}:${port}`;
  }

  public listDownloads(): GameData[] {
    try {
      const response = this.request.get(`${this.url}/downloads`);
      return JSON.parse(response.text);
    } catch {
      console.log("[PS2NetworkClient]: Failed to list downloads");
      return [];
    }
  }

  public search(key: string): GameData[] {
    try {
      const response = this.request.get(`${this.url}/search?key=${key}`);
      return JSON.parse(response.text);
    } catch {
      console.log("[PS2NetworkClient]: Failed to search");
      return [];
    }
  }
}
