import re
# from torch.nn import modules
from torchtext.nn import modules 
# from tensorflow.keras import layers
import torch
import inspect
import importlib
import tensorflow
from pprint import pprint
import ast
import subprocess
from string import Template
import json
layer_nodes_template = Template(
    """
/// $class_name
/// generate by convert_torch_nn_layer.py
export const $class_name =  defineLayerNode({
    node_name: "$class_name",
    params: {
$parse_params
        },
    pytorch: {
        class_name: "$class_name",
        module: "$modules_path",
        params: {
$params
        },
    },
});
    """
)

lazy_layer_nodes_template = Template(
    """ 
export const $module_name = defineLazyLayerNode({
    module_name: '$module_name',
    params: {
        $parse_params
    }
    ,
    options: 
$options
    ,
    nodes: 
$nodes
    ,
});
    """
)


def extract_params_torch_nn_layer(layer_class) -> str:
    init_function = layer_class.__init__
    y = inspect.getfullargspec(init_function)

    class_name= layer_class.__name__
    modules_path = layer_class.__module__
    annotations_list = [
        arg_name
        for arg_name in y.annotations
        if (arg_name != "self" and arg_name != "return")
    ]
    __params_class_args = [
        convert_print_arg(arg_name, str(y.annotations[arg_name]))
        for arg_name in annotations_list
    ]
    __pytorch_class_args = [
        f"{arg_name} : \"{str(y.annotations[arg_name])}\","
        for arg_name in annotations_list
    ]
    return {
        "class_name": class_name,
        "modules_path": modules_path,
        "params": __pytorch_class_args,
        "parse_params": __params_class_args,
    }

def convert_torch_nn_layer(layer_class) -> str:
    formatted_params = extract_params_torch_nn_layer(layer_class)
    formatted_params['params'] = "\n".join(formatted_params['params'])
    formatted_params['parse_params'] = "\n".join(formatted_params['parse_params'])
    return_print = layer_nodes_template.substitute(formatted_params)
    return return_print


def parse_simple_type(arg_type):
    if arg_type.startswith("typing"):
        return None
    match arg_type:
        case "str" | "<class 'str'>":
            return f"zod.string()"
        case 'int' | "<class 'int'>":
            return f'zod.number()'
        case 'float' | "<class 'float'>":
            return f'zod.number()'
        case 'bool' | "<class 'bool'>":
            return f'zod.boolean()'
        case 'torch.Tensor' | "<class 'torch.Tensor'>":
            return f'zod.array(zod.number())'
        case 'NoneType' | "<class 'NoneType'>":
            return f"zod.nullable()"
        case _ :
            return "zod.any()"

def ast_parse_rewrite(ast_obj) -> str:
    # print('-===-')
    # print(ast_obj)
    # print(ast.dump(ast_obj, indent=2))
    # print("-===-")

    if isinstance(ast_obj, ast.Name):
        return parse_simple_type(ast_obj.id) + ", "
    if isinstance(ast_obj, ast.Subscript):
        attr = ast_obj.value.attr
        # print('in sub', attr)

        if attr == 'Optional':
            # print("out sub", attr)
            if isinstance(ast_obj.slice, ast.Attribute):
                return f"zod.optional({parse_simple_type(ast_obj.slice.attr)}) "
            elif isinstance(ast_obj.slice, ast.Subscript):
                return f"zod.optional({ast_parse_rewrite(ast_obj.slice)}) "
            return f"zod.optional({parse_simple_type(ast_obj.slice.id)}) "
        
        elif attr == 'Union':
            # print("out sub", attr)
            return f"zod.union([{', '.join([ast_parse_rewrite(elt) for elt in ast_obj.slice.elts])}]) ".replace(
                ", ,", " ,"
            )
        # return f"{attr}<{ast_parse_rewrite(ast.slice.value)}>"
        elif attr == 'List' or attr == 'Sequence':
            # print("out sub", attr)
            return f"zod.array({parse_simple_type(ast_obj.slice.id)}) "
        elif attr == 'Tuple':
            # print("out sub", attr)
            if isinstance(ast_obj.slice, ast.Tuple): 
                return f"zod.tuple([{', '.join([ast_parse_rewrite(elt) for elt in ast_obj.slice.elts])}]) ".replace(', ,', ' ,')
            else:
                return f"zod.tuple([{ast_parse_rewrite(ast_obj.slice)}]) "

        elif attr == 'Callable':
            return f"zod.function()"

    print("unsupported ast type", ast_obj)
    return "zod.any()"

def parse_optional_type(arg_type):
    ast_parsed = ast.parse(arg_type)
    # print(ast.dump(ast_parsed , indent=2))
    reform = ast_parse_rewrite(ast_parsed.body[0].value)

    # replace the nullable with appending .nullable()
    has_nullable =  re.search(r"zod\.nullable\(\)\s*,", reform)
    if has_nullable:
        reform = reform.replace(has_nullable.group(0), "")
        reform = f"{reform}.nullable()"

    return reform


def convert_print_arg(arg_name, arg_type) -> str:
   
    parsed_type = parse_simple_type(arg_type)
    if not parsed_type:
        parsed_type = parse_optional_type(arg_type)

    return f'{arg_name}: {parsed_type},'


def batch_convert_torch_nn_layers():
    class_list = [
        getattr(modules, a)
        for a in dir(modules)
        if inspect.isclass(getattr(modules, a))
    ]
    class_list.sort(key=lambda x: str(x.__module__))
    # print("-------------------")
    # return [convert_torch_nn_layer(layer_class) for layer_class in layer_classes]
    return class_list

def last_element_of_path(path):
    return path.split('.')[-1]

def convert_torch_nn_layer_module(module_name, layers):
    layer_params_sets = [extract_params_torch_nn_layer(layer) for layer in layers]

    layer_params_shortened={}
    for layer_params_set in layer_params_sets:
        for pair_set in layer_params_set["parse_params"]:
            key_set = pair_set.split(": ")
            layer_params_shortened[key_set[0]] = key_set[1]

    layer_params_resets = [
            f"{keys} : {layer_params_shortened[keys]}"
            for keys in layer_params_shortened
        ]

    filtered_options = [
            f'{layer.__name__}' 
            for layer in layers 
            if str(layer.__name__).startswith("Lazy") == False
        ]

    module_params_set = {
            "module_name": module_name,
            "parse_params": "\n".join( layer_params_resets),
            "options": filtered_options,
            "nodes":  layer_params_sets,
        }

    return            lazy_layer_nodes_template.substitute(module_params_set)

def torch_main():
    batch_list = batch_convert_torch_nn_layers()

    generate_file = open(
        "../../app/src/node_graph/nodes/layer/torch__generated.ts", "w+"
    )
    generate_file.writelines(["import { defineLayerNode } from './base';" , "import * as zod from 'zod';"])
    for layer in batch_list:
        generate_file.writelines(convert_torch_nn_layer(layer))          

    generate_file.close()
    subprocess.run(["deno", "fmt", "../../app/src/node_graph/nodes/layer/torch__generated.ts"])

    group_by_module = {}
    for layer in batch_list:
        module_short_name = last_element_of_path(layer.__module__)
        if module_short_name not in group_by_module:
            group_by_module[module_short_name] = []
        group_by_module[module_short_name].append(layer)

    generate_file = open(
        "../../app/src/node_graph/nodes/layer/torch__generated-lazy.ts", "w+"
    )
    generate_file.writelines(["import { defineLazyLayerNode } from './base';" , "import * as zod from 'zod';"])
    for module_name, layers in group_by_module.items():
        generate_file.writelines( convert_torch_nn_layer_module(module_name, layers))
    generate_file.close()
    subprocess.run(
        ["deno", "fmt", "../../app/src/node_graph/nodes/layer/torch__generated-lazy.ts"]
    )

    # import mod file

    mod_file = open("../../app/src/node_graph/nodes/layer/mod.ts", "w+")
    mod_file.writelines("import {" )
    mod_file.writelines(
        [layer.__name__ + " as torch__" + layer.__name__ + " ," for layer in batch_list]
    )
    mod_file.writelines("} from './torch__generated';\n")

    mod_file.writelines("export default {\n")
    mod_file.writelines(
        ['torch__'+layer.__name__+',' for layer in batch_list]
    )
    mod_file.writelines("\n};")

    mod_file.writelines(
        "export const initNodeModules = (baklava_app) => {",
    )
    mod_file.writelines(
        [
            "baklava_app.editor.registerNodeType(torch__{}, {});\n".format(
                layer.__name__,
                '{ category: "torch/' + last_element_of_path(layer.__module__) + '" }',
            )
            for layer in batch_list
        ]
    )
    mod_file.writelines("};")
    mod_file.close()
    subprocess.run(["deno", "fmt", "../../app/src/node_graph/nodes/layer/mod.ts"])

    print("Done!")

# ---------------------------------

def batch_convert_tfk_layers():
    class_list = [
        getattr(layers, a)
        for a in dir(layers)
        if inspect.isclass(getattr(layers, a))
    ]
    class_list.sort(key=lambda x: str(x.__module__))
    # print("-------------------")
    # return [convert_torch_nn_layer(layer_class) for layer_class in layer_classes]
    return class_list





if __name__ == "__main__":
    torch_main()
    # tf_main()