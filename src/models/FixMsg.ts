interface FixMsg {
    id: number,
    values: Field[],
}
  
interface Field {
    tag: string,
    title: string,
    value: string,
}