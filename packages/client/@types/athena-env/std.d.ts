declare interface File {
  close(): number;
  readAsString(size?: number): string;
}

declare const std: {
  exists(filename: string): boolean;
  open(filename: string, flags: string, errorObj?: Error): File;
}
