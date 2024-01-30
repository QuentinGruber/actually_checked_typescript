export function addMaybe(a: number | undefined, b?: number): number {
  if (a === undefined) {
    return 0;
  }
  if (b === undefined) {
    return a;
  }
  return a + b;
}
