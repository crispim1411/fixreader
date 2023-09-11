interface FixMsg {
    id: number,
    fields: Field[],
}
  
interface Field {
    tag: string,
    value: string,
}