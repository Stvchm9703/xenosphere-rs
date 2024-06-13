// import  {Node} from "@baklavajs/core/dist/node";
import { setType, NodeInterface, defineDynamicNode, TextInterface, NumberInterface, CheckboxInterface, SelectInterface, BaklavaInterfaceTypes, NodeInterfaceType } from "baklavajs";
// import { setType } from "@baklavajs/interface-types";
import * as zod from "zod";


export type PythonRawParams = {
  [key: string]: string;
}

export type PythonTransformParams = {
  [key: string]: zod.ZodType<any, any, any>;
}
export type TensorObject<T> = T;

// 
const TensorObjectDynDefine = zod.union([
  // /** 1D */ zod.number(),
  /** 2D */ zod.array(zod.number()),
  /** 3D */ zod.array(zod.array(zod.number())),
  /** 4D */ zod.array(zod.array(zod.array(zod.number()))),
  /** 4D */ zod.array(zod.array(zod.array(zod.array(zod.number())))),
]);

export type TensorObjectDyn = zod.infer<typeof TensorObjectDynDefine>;

export const TensorObjectDynInterfaceType = new NodeInterfaceType<TensorObjectDyn>("Tensor Object");


export type LayerNodeInterface = {
  node_name: string;
  params: PythonTransformParams;
  pytorch: {
    class_name: string;
    module: string;
    params: PythonRawParams;
  };
  // tensorflow: {
  //   class_name: string;
  //   module: string;
  // };
  // onnx: {
  //   class_name: string;
  //   module: string;
  // };

}

const mappingInput = (args: { node_name: string, params: PythonTransformParams }) => {
  // re-calibrate the output channel based on the node_class and params setting 
  // e.g. Conv2D, Dense, etc.
  // console.group("mappingInput")
  // console.log("args", args)
  if (args.node_name === "Input") {

  }

  // other params setting
  const input_params = Object.entries(args.params).map(([key, value]) => {
    // console.log("key", key)
    // console.log("value", value)
    let _interface = null;
    if (value._def.typeName === "ZodString") {
      _interface = () => new TextInterface(key, "")
    } else if (value._def.typeName === "ZodNumber") {
      _interface = () => new NumberInterface(key, 0.0)
    } else if (value._def.typeName === "ZodBoolean") {
      _interface = () => new CheckboxInterface(key, false)
    } else {
      _interface = () => new NodeInterface<number>(key, 0)
    }
    return {
      [key]: _interface
    }
  }).flat().reduce((acc, val) => Object.assign(acc, val), {});
  // console.groupEnd();
  return {
    input_tensor: () => new NodeInterface<TensorObjectDyn>("Input Tensor", [[0]]).use(setType, TensorObjectDynInterfaceType),
    ...input_params
  }
}

export const defineLayerNode = (definition: LayerNodeInterface) => defineDynamicNode({
  title: definition.node_name,
  type: definition.node_name,
  // type: "LayerNode",
  _pytorch: definition.pytorch,
  _de: definition,
  inputs: mappingInput(definition),
  outputs: {
    // generic output
    output_tensor: () => new NodeInterface<TensorObjectDyn>("Output Tensor", [[0]]).use(setType, TensorObjectDynInterfaceType),
  },
  onUpdate: (inputs, outputs) => {
    console.log("onUpdate", inputs, outputs)
    const IO_set = {
      inputs: mappingInput(definition),
      outputs: {
        output_tensor: () => new NodeInterface<TensorObjectDyn>("Output Tensor", [[0]]).use(setType, TensorObjectDynInterfaceType),
      }
    }
    console.log("IO_set", IO_set);
    return IO_set;
  },
  // onCreate: () => {
  //   console.log("onCreate")
  // },

});

export type LazyLayerNodeInterface = {
  module_name: string;
  params: PythonTransformParams;
  options: Array<string>;
  nodes: Array<{
    class_name: string;
    modules_path: string;
    // module: string;
    params: Array<string>;
    parse_params: Array<string>;
  }>;
  // tensorflow: {
  //   class_name: string;
  //   module: string;
  // };
  // onnx: {
  //   class_name: string;
  //   module: string;
  // };

}



export const defineLazyLayerNode = (definition: LazyLayerNodeInterface) => {
  const input_tensor_interface = new NodeInterface<TensorObjectDyn>("Input Tensor", [[0]]).use(setType, TensorObjectDynInterfaceType);
  const output_tensor_interface = new NodeInterface<TensorObjectDyn>("Output Tensor", [[0]]).use(setType, TensorObjectDynInterfaceType);
  return defineDynamicNode({
    title: definition.module_name,
    type: definition.module_name,
    // type: "LayerNode",
    // _pytorch: definition.pytorch,
    _de: definition,
    inputs: {
      input_tensor: () => input_tensor_interface,
      function_option: () => new SelectInterface<string>("function", "", definition.options).setPort(false),
    },
    outputs: {
      // generic output
      output_tensor: () => output_tensor_interface,
    },
    onUpdate: (inputs, outputs) => {
      console.log("onUpdate", inputs, outputs);
      console.log(definition)
      return {
        inputs: {
          input_tensor: () => input_tensor_interface,
        },
        outputs: {
          output_tensor: () => output_tensor_interface,
        }
      }
    },
    calculate: (inputs) => {
      console.log("calculate", inputs)
      return {
        output_tensor: [[0]]
      }
    }
    // onCreate: () => {
    //   console.log("onCreate")
    // },

  });
}
