import { IntegerInterface, NodeInterface, NumberInterface, defineDynamicNode } from "baklavajs";


export type TupleValue = number[];
export const ValueNode = defineDynamicNode({
  title: "Value",
  type: 'value_node',
  inputs: {
    dim: () => new IntegerInterface("dim", 1, 1, 10),
  },
  outputs: {
    output_value: () => new NodeInterface<TupleValue>("tuple set", []),
  },
  onUpdate(inputs) {
    let fields:any = {};
    for (let i = 0; i < inputs.dim; i++) {
      fields[`val_${i}`] = ()=> new NumberInterface(`layer ${i}`, 0);
    }
    const IO_set =  {
      inputs:{
        dim: () => new IntegerInterface("dim", 1, 1, 10),
        ...fields
      },
      // outputs: {
      //   output_value: () => new NodeInterface<TupleValue>("tuple set", fields.map((f:any)=>f.value)),
      // },
      forceUpdateInputs: ['dim', ...Object.keys(fields)]
    }
    console.log('onUpdate', IO_set);  
    return IO_set;
  },
  calculate(inputs) {
    console.log('calculate', inputs);
    let val_set = Object.entries(inputs)
      .filter(([k, v])=>k.startsWith('val_'))
      .map(([_, v])=>v);
    return {
      output_value: val_set
    }
  }
});