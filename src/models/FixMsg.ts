interface FixMsg {
    id: number,
    values: Field[],
}
  
interface Field {
    tag: string,
    value: string,
}