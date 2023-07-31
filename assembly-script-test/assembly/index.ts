// The entry file of your WebAssembly module.
@external("host", "init")
declare function init(magicString: string): void;
@external("host", "get")
declare function get(thing: string, len: i32): void;

export function tacocat(): void {
  init("orb")
  const thing = "tacocat"
  get(thing, thing.length)
}

export function talk(num: number): string {
  return `<h2>Hello, World! {$num}</h2>`
}

// export function racecar(): string {
//   let num = response("hello");
//   return tacocat(num);
// }
