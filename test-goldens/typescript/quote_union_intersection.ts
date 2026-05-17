type StringOrNumber = string | number;
type Combined = TypeA & TypeB;
function process(value: string | number | null): void {
  console.log(value);
}
