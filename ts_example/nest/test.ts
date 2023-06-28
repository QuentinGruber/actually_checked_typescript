type int = number;

function add(a: number, b: int, c: string, d: bigint): number {
  return a + b;
}

function concat(a: string, b: string): string {
  return a + b;
}

export function concat2(a: string = "d", b: string): string {
  return a + b;
}

class classo {
  name: string;
  constructor(name: string) {
    this.name = name;
  }

  toLower(name: string): string {
    return name.toLowerCase();
  }
}

export class classo2 {
  name: string;
  constructor(name: string) {
    this.name = name;
  }

  toLower(name: string): string {
    return name.toLowerCase();
  }
}

const concat3 = (a: string, b: string): string => {
  return a + b;
};

(a: string, b: string): string => {
  return a + b;
};
