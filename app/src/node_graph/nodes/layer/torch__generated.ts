import { defineLayerNode } from "./base";
import * as zod from "zod";
/// CELU
/// generate by convert_torch_nn_layer.py
export const CELU = defineLayerNode({
  node_name: "CELU",
  params: {
    alpha: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "CELU",
    module: "torch.nn.modules.activation",
    params: {
      alpha: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// ELU
/// generate by convert_torch_nn_layer.py
export const ELU = defineLayerNode({
  node_name: "ELU",
  params: {
    alpha: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "ELU",
    module: "torch.nn.modules.activation",
    params: {
      alpha: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// GELU
/// generate by convert_torch_nn_layer.py
export const GELU = defineLayerNode({
  node_name: "GELU",
  params: {
    approximate: zod.string(),
  },
  pytorch: {
    class_name: "GELU",
    module: "torch.nn.modules.activation",
    params: {
      approximate: "<class 'str'>",
    },
  },
});

/// GLU
/// generate by convert_torch_nn_layer.py
export const GLU = defineLayerNode({
  node_name: "GLU",
  params: {
    dim: zod.number(),
  },
  pytorch: {
    class_name: "GLU",
    module: "torch.nn.modules.activation",
    params: {
      dim: "<class 'int'>",
    },
  },
});

/// Hardshrink
/// generate by convert_torch_nn_layer.py
export const Hardshrink = defineLayerNode({
  node_name: "Hardshrink",
  params: {
    lambd: zod.number(),
  },
  pytorch: {
    class_name: "Hardshrink",
    module: "torch.nn.modules.activation",
    params: {
      lambd: "<class 'float'>",
    },
  },
});

/// Hardsigmoid
/// generate by convert_torch_nn_layer.py
export const Hardsigmoid = defineLayerNode({
  node_name: "Hardsigmoid",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Hardsigmoid",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// Hardswish
/// generate by convert_torch_nn_layer.py
export const Hardswish = defineLayerNode({
  node_name: "Hardswish",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Hardswish",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// Hardtanh
/// generate by convert_torch_nn_layer.py
export const Hardtanh = defineLayerNode({
  node_name: "Hardtanh",
  params: {
    min_val: zod.number(),
    max_val: zod.number(),
    inplace: zod.boolean(),
    min_value: zod.optional(zod.number()),
    max_value: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "Hardtanh",
    module: "torch.nn.modules.activation",
    params: {
      min_val: "<class 'float'>",
      max_val: "<class 'float'>",
      inplace: "<class 'bool'>",
      min_value: "typing.Optional[float]",
      max_value: "typing.Optional[float]",
    },
  },
});

/// LeakyReLU
/// generate by convert_torch_nn_layer.py
export const LeakyReLU = defineLayerNode({
  node_name: "LeakyReLU",
  params: {
    negative_slope: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "LeakyReLU",
    module: "torch.nn.modules.activation",
    params: {
      negative_slope: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// LogSigmoid
/// generate by convert_torch_nn_layer.py
export const LogSigmoid = defineLayerNode({
  node_name: "LogSigmoid",
  params: {},
  pytorch: {
    class_name: "LogSigmoid",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// LogSoftmax
/// generate by convert_torch_nn_layer.py
export const LogSoftmax = defineLayerNode({
  node_name: "LogSoftmax",
  params: {
    dim: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "LogSoftmax",
    module: "torch.nn.modules.activation",
    params: {
      dim: "typing.Optional[int]",
    },
  },
});

/// Mish
/// generate by convert_torch_nn_layer.py
export const Mish = defineLayerNode({
  node_name: "Mish",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Mish",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// MultiheadAttention
/// generate by convert_torch_nn_layer.py
export const MultiheadAttention = defineLayerNode({
  node_name: "MultiheadAttention",
  params: {},
  pytorch: {
    class_name: "MultiheadAttention",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// PReLU
/// generate by convert_torch_nn_layer.py
export const PReLU = defineLayerNode({
  node_name: "PReLU",
  params: {
    num_parameters: zod.number(),
    init: zod.number(),
  },
  pytorch: {
    class_name: "PReLU",
    module: "torch.nn.modules.activation",
    params: {
      num_parameters: "<class 'int'>",
      init: "<class 'float'>",
    },
  },
});

/// RReLU
/// generate by convert_torch_nn_layer.py
export const RReLU = defineLayerNode({
  node_name: "RReLU",
  params: {
    lower: zod.number(),
    upper: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "RReLU",
    module: "torch.nn.modules.activation",
    params: {
      lower: "<class 'float'>",
      upper: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// ReLU
/// generate by convert_torch_nn_layer.py
export const ReLU = defineLayerNode({
  node_name: "ReLU",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "ReLU",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// ReLU6
/// generate by convert_torch_nn_layer.py
export const ReLU6 = defineLayerNode({
  node_name: "ReLU6",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "ReLU6",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// SELU
/// generate by convert_torch_nn_layer.py
export const SELU = defineLayerNode({
  node_name: "SELU",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "SELU",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// SiLU
/// generate by convert_torch_nn_layer.py
export const SiLU = defineLayerNode({
  node_name: "SiLU",
  params: {
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "SiLU",
    module: "torch.nn.modules.activation",
    params: {
      inplace: "<class 'bool'>",
    },
  },
});

/// Sigmoid
/// generate by convert_torch_nn_layer.py
export const Sigmoid = defineLayerNode({
  node_name: "Sigmoid",
  params: {},
  pytorch: {
    class_name: "Sigmoid",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// Softmax
/// generate by convert_torch_nn_layer.py
export const Softmax = defineLayerNode({
  node_name: "Softmax",
  params: {
    dim: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "Softmax",
    module: "torch.nn.modules.activation",
    params: {
      dim: "typing.Optional[int]",
    },
  },
});

/// Softmax2d
/// generate by convert_torch_nn_layer.py
export const Softmax2d = defineLayerNode({
  node_name: "Softmax2d",
  params: {},
  pytorch: {
    class_name: "Softmax2d",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// Softmin
/// generate by convert_torch_nn_layer.py
export const Softmin = defineLayerNode({
  node_name: "Softmin",
  params: {
    dim: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "Softmin",
    module: "torch.nn.modules.activation",
    params: {
      dim: "typing.Optional[int]",
    },
  },
});

/// Softplus
/// generate by convert_torch_nn_layer.py
export const Softplus = defineLayerNode({
  node_name: "Softplus",
  params: {
    beta: zod.number(),
    threshold: zod.number(),
  },
  pytorch: {
    class_name: "Softplus",
    module: "torch.nn.modules.activation",
    params: {
      beta: "<class 'int'>",
      threshold: "<class 'int'>",
    },
  },
});

/// Softshrink
/// generate by convert_torch_nn_layer.py
export const Softshrink = defineLayerNode({
  node_name: "Softshrink",
  params: {
    lambd: zod.number(),
  },
  pytorch: {
    class_name: "Softshrink",
    module: "torch.nn.modules.activation",
    params: {
      lambd: "<class 'float'>",
    },
  },
});

/// Softsign
/// generate by convert_torch_nn_layer.py
export const Softsign = defineLayerNode({
  node_name: "Softsign",
  params: {},
  pytorch: {
    class_name: "Softsign",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// Tanh
/// generate by convert_torch_nn_layer.py
export const Tanh = defineLayerNode({
  node_name: "Tanh",
  params: {},
  pytorch: {
    class_name: "Tanh",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// Tanhshrink
/// generate by convert_torch_nn_layer.py
export const Tanhshrink = defineLayerNode({
  node_name: "Tanhshrink",
  params: {},
  pytorch: {
    class_name: "Tanhshrink",
    module: "torch.nn.modules.activation",
    params: {},
  },
});

/// Threshold
/// generate by convert_torch_nn_layer.py
export const Threshold = defineLayerNode({
  node_name: "Threshold",
  params: {
    threshold: zod.number(),
    value: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Threshold",
    module: "torch.nn.modules.activation",
    params: {
      threshold: "<class 'float'>",
      value: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// AdaptiveLogSoftmaxWithLoss
/// generate by convert_torch_nn_layer.py
export const AdaptiveLogSoftmaxWithLoss = defineLayerNode({
  node_name: "AdaptiveLogSoftmaxWithLoss",
  params: {
    in_features: zod.number(),
    n_classes: zod.number(),
    cutoffs: zod.array(zod.number()),
    div_value: zod.number(),
    head_bias: zod.boolean(),
  },
  pytorch: {
    class_name: "AdaptiveLogSoftmaxWithLoss",
    module: "torch.nn.modules.adaptive",
    params: {
      in_features: "<class 'int'>",
      n_classes: "<class 'int'>",
      cutoffs: "typing.Sequence[int]",
      div_value: "<class 'float'>",
      head_bias: "<class 'bool'>",
    },
  },
});

/// BatchNorm1d
/// generate by convert_torch_nn_layer.py
export const BatchNorm1d = defineLayerNode({
  node_name: "BatchNorm1d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "BatchNorm1d",
    module: "torch.nn.modules.batchnorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// BatchNorm2d
/// generate by convert_torch_nn_layer.py
export const BatchNorm2d = defineLayerNode({
  node_name: "BatchNorm2d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "BatchNorm2d",
    module: "torch.nn.modules.batchnorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// BatchNorm3d
/// generate by convert_torch_nn_layer.py
export const BatchNorm3d = defineLayerNode({
  node_name: "BatchNorm3d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "BatchNorm3d",
    module: "torch.nn.modules.batchnorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// LazyBatchNorm1d
/// generate by convert_torch_nn_layer.py
export const LazyBatchNorm1d = defineLayerNode({
  node_name: "LazyBatchNorm1d",
  params: {},
  pytorch: {
    class_name: "LazyBatchNorm1d",
    module: "torch.nn.modules.batchnorm",
    params: {},
  },
});

/// LazyBatchNorm2d
/// generate by convert_torch_nn_layer.py
export const LazyBatchNorm2d = defineLayerNode({
  node_name: "LazyBatchNorm2d",
  params: {},
  pytorch: {
    class_name: "LazyBatchNorm2d",
    module: "torch.nn.modules.batchnorm",
    params: {},
  },
});

/// LazyBatchNorm3d
/// generate by convert_torch_nn_layer.py
export const LazyBatchNorm3d = defineLayerNode({
  node_name: "LazyBatchNorm3d",
  params: {},
  pytorch: {
    class_name: "LazyBatchNorm3d",
    module: "torch.nn.modules.batchnorm",
    params: {},
  },
});

/// SyncBatchNorm
/// generate by convert_torch_nn_layer.py
export const SyncBatchNorm = defineLayerNode({
  node_name: "SyncBatchNorm",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
    process_group: zod.optional(zod.any()),
  },
  pytorch: {
    class_name: "SyncBatchNorm",
    module: "torch.nn.modules.batchnorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
      process_group: "typing.Optional[typing.Any]",
    },
  },
});

/// ChannelShuffle
/// generate by convert_torch_nn_layer.py
export const ChannelShuffle = defineLayerNode({
  node_name: "ChannelShuffle",
  params: {
    groups: zod.number(),
  },
  pytorch: {
    class_name: "ChannelShuffle",
    module: "torch.nn.modules.channelshuffle",
    params: {
      groups: "<class 'int'>",
    },
  },
});

/// Container
/// generate by convert_torch_nn_layer.py
export const Container = defineLayerNode({
  node_name: "Container",
  params: {
    kwargs: zod.any(),
  },
  pytorch: {
    class_name: "Container",
    module: "torch.nn.modules.container",
    params: {
      kwargs: "typing.Any",
    },
  },
});

/// ModuleDict
/// generate by convert_torch_nn_layer.py
export const ModuleDict = defineLayerNode({
  node_name: "ModuleDict",
  params: {
    modules: zod.optional(zod.any()),
  },
  pytorch: {
    class_name: "ModuleDict",
    module: "torch.nn.modules.container",
    params: {
      modules:
        "typing.Optional[typing.Mapping[str, torch.nn.modules.module.Module]]",
    },
  },
});

/// ModuleList
/// generate by convert_torch_nn_layer.py
export const ModuleList = defineLayerNode({
  node_name: "ModuleList",
  params: {
    modules: zod.optional(zod.any()),
  },
  pytorch: {
    class_name: "ModuleList",
    module: "torch.nn.modules.container",
    params: {
      modules:
        "typing.Optional[typing.Iterable[torch.nn.modules.module.Module]]",
    },
  },
});

/// ParameterDict
/// generate by convert_torch_nn_layer.py
export const ParameterDict = defineLayerNode({
  node_name: "ParameterDict",
  params: {
    parameters: zod.any(),
  },
  pytorch: {
    class_name: "ParameterDict",
    module: "torch.nn.modules.container",
    params: {
      parameters: "typing.Any",
    },
  },
});

/// ParameterList
/// generate by convert_torch_nn_layer.py
export const ParameterList = defineLayerNode({
  node_name: "ParameterList",
  params: {
    values: zod.optional(zod.any()),
  },
  pytorch: {
    class_name: "ParameterList",
    module: "torch.nn.modules.container",
    params: {
      values: "typing.Optional[typing.Iterable[typing.Any]]",
    },
  },
});

/// Sequential
/// generate by convert_torch_nn_layer.py
export const Sequential = defineLayerNode({
  node_name: "Sequential",
  params: {},
  pytorch: {
    class_name: "Sequential",
    module: "torch.nn.modules.container",
    params: {},
  },
});

/// Conv1d
/// generate by convert_torch_nn_layer.py
export const Conv1d = defineLayerNode({
  node_name: "Conv1d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding: zod.union([zod.string(), zod.number(), zod.tuple([zod.number()])]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number()])]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "Conv1d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int]]",
      padding: "typing.Union[str, int, typing.Tuple[int]]",
      dilation: "typing.Union[int, typing.Tuple[int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// Conv2d
/// generate by convert_torch_nn_layer.py
export const Conv2d = defineLayerNode({
  node_name: "Conv2d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    padding: zod.union([
      zod.string(),
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "Conv2d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int]]",
      padding: "typing.Union[str, int, typing.Tuple[int, int]]",
      dilation: "typing.Union[int, typing.Tuple[int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// Conv3d
/// generate by convert_torch_nn_layer.py
export const Conv3d = defineLayerNode({
  node_name: "Conv3d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding: zod.union([
      zod.string(),
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "Conv3d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding: "typing.Union[str, int, typing.Tuple[int, int, int]]",
      dilation: "typing.Union[int, typing.Tuple[int, int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// ConvTranspose1d
/// generate by convert_torch_nn_layer.py
export const ConvTranspose1d = defineLayerNode({
  node_name: "ConvTranspose1d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    output_padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "ConvTranspose1d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int]]",
      padding: "typing.Union[int, typing.Tuple[int]]",
      output_padding: "typing.Union[int, typing.Tuple[int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "typing.Union[int, typing.Tuple[int]]",
      padding_mode: "<class 'str'>",
    },
  },
});

/// ConvTranspose2d
/// generate by convert_torch_nn_layer.py
export const ConvTranspose2d = defineLayerNode({
  node_name: "ConvTranspose2d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    output_padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "ConvTranspose2d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int]]",
      output_padding: "typing.Union[int, typing.Tuple[int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "typing.Union[int, typing.Tuple[int, int]]",
      padding_mode: "<class 'str'>",
    },
  },
});

/// ConvTranspose3d
/// generate by convert_torch_nn_layer.py
export const ConvTranspose3d = defineLayerNode({
  node_name: "ConvTranspose3d",
  params: {
    in_channels: zod.number(),
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    output_padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "ConvTranspose3d",
    module: "torch.nn.modules.conv",
    params: {
      in_channels: "<class 'int'>",
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      output_padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConv1d
/// generate by convert_torch_nn_layer.py
export const LazyConv1d = defineLayerNode({
  node_name: "LazyConv1d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number()])]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConv1d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int]]",
      padding: "typing.Union[int, typing.Tuple[int]]",
      dilation: "typing.Union[int, typing.Tuple[int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConv2d
/// generate by convert_torch_nn_layer.py
export const LazyConv2d = defineLayerNode({
  node_name: "LazyConv2d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConv2d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int]]",
      dilation: "typing.Union[int, typing.Tuple[int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConv3d
/// generate by convert_torch_nn_layer.py
export const LazyConv3d = defineLayerNode({
  node_name: "LazyConv3d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConv3d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      dilation: "typing.Union[int, typing.Tuple[int, int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConvTranspose1d
/// generate by convert_torch_nn_layer.py
export const LazyConvTranspose1d = defineLayerNode({
  node_name: "LazyConvTranspose1d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    output_padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConvTranspose1d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int]]",
      padding: "typing.Union[int, typing.Tuple[int]]",
      output_padding: "typing.Union[int, typing.Tuple[int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "typing.Union[int, typing.Tuple[int]]",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConvTranspose2d
/// generate by convert_torch_nn_layer.py
export const LazyConvTranspose2d = defineLayerNode({
  node_name: "LazyConvTranspose2d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    output_padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.number(),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConvTranspose2d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int]]",
      output_padding: "typing.Union[int, typing.Tuple[int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "<class 'int'>",
      padding_mode: "<class 'str'>",
    },
  },
});

/// LazyConvTranspose3d
/// generate by convert_torch_nn_layer.py
export const LazyConvTranspose3d = defineLayerNode({
  node_name: "LazyConvTranspose3d",
  params: {
    out_channels: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    output_padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    groups: zod.number(),
    bias: zod.boolean(),
    dilation: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    padding_mode: zod.string(),
  },
  pytorch: {
    class_name: "LazyConvTranspose3d",
    module: "torch.nn.modules.conv",
    params: {
      out_channels: "<class 'int'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      output_padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      groups: "<class 'int'>",
      bias: "<class 'bool'>",
      dilation: "typing.Union[int, typing.Tuple[int, int, int]]",
      padding_mode: "<class 'str'>",
    },
  },
});

/// CosineSimilarity
/// generate by convert_torch_nn_layer.py
export const CosineSimilarity = defineLayerNode({
  node_name: "CosineSimilarity",
  params: {
    dim: zod.number(),
    eps: zod.number(),
  },
  pytorch: {
    class_name: "CosineSimilarity",
    module: "torch.nn.modules.distance",
    params: {
      dim: "<class 'int'>",
      eps: "<class 'float'>",
    },
  },
});

/// PairwiseDistance
/// generate by convert_torch_nn_layer.py
export const PairwiseDistance = defineLayerNode({
  node_name: "PairwiseDistance",
  params: {
    p: zod.number(),
    eps: zod.number(),
    keepdim: zod.boolean(),
  },
  pytorch: {
    class_name: "PairwiseDistance",
    module: "torch.nn.modules.distance",
    params: {
      p: "<class 'float'>",
      eps: "<class 'float'>",
      keepdim: "<class 'bool'>",
    },
  },
});

/// AlphaDropout
/// generate by convert_torch_nn_layer.py
export const AlphaDropout = defineLayerNode({
  node_name: "AlphaDropout",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "AlphaDropout",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// Dropout
/// generate by convert_torch_nn_layer.py
export const Dropout = defineLayerNode({
  node_name: "Dropout",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Dropout",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// Dropout1d
/// generate by convert_torch_nn_layer.py
export const Dropout1d = defineLayerNode({
  node_name: "Dropout1d",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Dropout1d",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// Dropout2d
/// generate by convert_torch_nn_layer.py
export const Dropout2d = defineLayerNode({
  node_name: "Dropout2d",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Dropout2d",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// Dropout3d
/// generate by convert_torch_nn_layer.py
export const Dropout3d = defineLayerNode({
  node_name: "Dropout3d",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "Dropout3d",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// FeatureAlphaDropout
/// generate by convert_torch_nn_layer.py
export const FeatureAlphaDropout = defineLayerNode({
  node_name: "FeatureAlphaDropout",
  params: {
    p: zod.number(),
    inplace: zod.boolean(),
  },
  pytorch: {
    class_name: "FeatureAlphaDropout",
    module: "torch.nn.modules.dropout",
    params: {
      p: "<class 'float'>",
      inplace: "<class 'bool'>",
    },
  },
});

/// Flatten
/// generate by convert_torch_nn_layer.py
export const Flatten = defineLayerNode({
  node_name: "Flatten",
  params: {
    start_dim: zod.number(),
    end_dim: zod.number(),
  },
  pytorch: {
    class_name: "Flatten",
    module: "torch.nn.modules.flatten",
    params: {
      start_dim: "<class 'int'>",
      end_dim: "<class 'int'>",
    },
  },
});

/// Unflatten
/// generate by convert_torch_nn_layer.py
export const Unflatten = defineLayerNode({
  node_name: "Unflatten",
  params: {
    dim: zod.union([zod.number(), zod.string()]),
    unflattened_size: zod.union([
      zod.any(),
      zod.array(zod.number()),
      zod.tuple([zod.number(), zod.any()]),
      zod.tuple([zod.tuple([zod.string(), zod.number()])]),
    ]),
  },
  pytorch: {
    class_name: "Unflatten",
    module: "torch.nn.modules.flatten",
    params: {
      dim: "typing.Union[int, str]",
      unflattened_size:
        "typing.Union[torch.Size, typing.List[int], typing.Tuple[int, ...], typing.Tuple[typing.Tuple[str, int]]]",
    },
  },
});

/// Fold
/// generate by convert_torch_nn_layer.py
export const Fold = defineLayerNode({
  node_name: "Fold",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
  },
  pytorch: {
    class_name: "Fold",
    module: "torch.nn.modules.fold",
    params: {
      output_size: "typing.Union[int, typing.Tuple[int, ...]]",
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      dilation: "typing.Union[int, typing.Tuple[int, ...]]",
      padding: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...]]",
    },
  },
});

/// Unfold
/// generate by convert_torch_nn_layer.py
export const Unfold = defineLayerNode({
  node_name: "Unfold",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
  },
  pytorch: {
    class_name: "Unfold",
    module: "torch.nn.modules.fold",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      dilation: "typing.Union[int, typing.Tuple[int, ...]]",
      padding: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...]]",
    },
  },
});

/// InstanceNorm1d
/// generate by convert_torch_nn_layer.py
export const InstanceNorm1d = defineLayerNode({
  node_name: "InstanceNorm1d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "InstanceNorm1d",
    module: "torch.nn.modules.instancenorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// InstanceNorm2d
/// generate by convert_torch_nn_layer.py
export const InstanceNorm2d = defineLayerNode({
  node_name: "InstanceNorm2d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "InstanceNorm2d",
    module: "torch.nn.modules.instancenorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// InstanceNorm3d
/// generate by convert_torch_nn_layer.py
export const InstanceNorm3d = defineLayerNode({
  node_name: "InstanceNorm3d",
  params: {
    num_features: zod.number(),
    eps: zod.number(),
    momentum: zod.number(),
    affine: zod.boolean(),
    track_running_stats: zod.boolean(),
  },
  pytorch: {
    class_name: "InstanceNorm3d",
    module: "torch.nn.modules.instancenorm",
    params: {
      num_features: "<class 'int'>",
      eps: "<class 'float'>",
      momentum: "<class 'float'>",
      affine: "<class 'bool'>",
      track_running_stats: "<class 'bool'>",
    },
  },
});

/// LazyInstanceNorm1d
/// generate by convert_torch_nn_layer.py
export const LazyInstanceNorm1d = defineLayerNode({
  node_name: "LazyInstanceNorm1d",
  params: {},
  pytorch: {
    class_name: "LazyInstanceNorm1d",
    module: "torch.nn.modules.instancenorm",
    params: {},
  },
});

/// LazyInstanceNorm2d
/// generate by convert_torch_nn_layer.py
export const LazyInstanceNorm2d = defineLayerNode({
  node_name: "LazyInstanceNorm2d",
  params: {},
  pytorch: {
    class_name: "LazyInstanceNorm2d",
    module: "torch.nn.modules.instancenorm",
    params: {},
  },
});

/// LazyInstanceNorm3d
/// generate by convert_torch_nn_layer.py
export const LazyInstanceNorm3d = defineLayerNode({
  node_name: "LazyInstanceNorm3d",
  params: {},
  pytorch: {
    class_name: "LazyInstanceNorm3d",
    module: "torch.nn.modules.instancenorm",
    params: {},
  },
});

/// Bilinear
/// generate by convert_torch_nn_layer.py
export const Bilinear = defineLayerNode({
  node_name: "Bilinear",
  params: {
    in1_features: zod.number(),
    in2_features: zod.number(),
    out_features: zod.number(),
    bias: zod.boolean(),
  },
  pytorch: {
    class_name: "Bilinear",
    module: "torch.nn.modules.linear",
    params: {
      in1_features: "<class 'int'>",
      in2_features: "<class 'int'>",
      out_features: "<class 'int'>",
      bias: "<class 'bool'>",
    },
  },
});

/// Identity
/// generate by convert_torch_nn_layer.py
export const Identity = defineLayerNode({
  node_name: "Identity",
  params: {
    args: zod.any(),
    kwargs: zod.any(),
  },
  pytorch: {
    class_name: "Identity",
    module: "torch.nn.modules.linear",
    params: {
      args: "typing.Any",
      kwargs: "typing.Any",
    },
  },
});

/// LazyLinear
/// generate by convert_torch_nn_layer.py
export const LazyLinear = defineLayerNode({
  node_name: "LazyLinear",
  params: {
    out_features: zod.number(),
    bias: zod.boolean(),
  },
  pytorch: {
    class_name: "LazyLinear",
    module: "torch.nn.modules.linear",
    params: {
      out_features: "<class 'int'>",
      bias: "<class 'bool'>",
    },
  },
});

/// Linear
/// generate by convert_torch_nn_layer.py
export const Linear = defineLayerNode({
  node_name: "Linear",
  params: {
    in_features: zod.number(),
    out_features: zod.number(),
    bias: zod.boolean(),
  },
  pytorch: {
    class_name: "Linear",
    module: "torch.nn.modules.linear",
    params: {
      in_features: "<class 'int'>",
      out_features: "<class 'int'>",
      bias: "<class 'bool'>",
    },
  },
});

/// BCELoss
/// generate by convert_torch_nn_layer.py
export const BCELoss = defineLayerNode({
  node_name: "BCELoss",
  params: {
    weight: zod.optional(zod.any()),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "BCELoss",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      reduction: "<class 'str'>",
    },
  },
});

/// BCEWithLogitsLoss
/// generate by convert_torch_nn_layer.py
export const BCEWithLogitsLoss = defineLayerNode({
  node_name: "BCEWithLogitsLoss",
  params: {
    weight: zod.optional(zod.any()),
    reduction: zod.string(),
    pos_weight: zod.optional(zod.any()),
  },
  pytorch: {
    class_name: "BCEWithLogitsLoss",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      reduction: "<class 'str'>",
      pos_weight: "typing.Optional[torch.Tensor]",
    },
  },
});

/// CTCLoss
/// generate by convert_torch_nn_layer.py
export const CTCLoss = defineLayerNode({
  node_name: "CTCLoss",
  params: {
    blank: zod.number(),
    reduction: zod.string(),
    zero_infinity: zod.boolean(),
  },
  pytorch: {
    class_name: "CTCLoss",
    module: "torch.nn.modules.loss",
    params: {
      blank: "<class 'int'>",
      reduction: "<class 'str'>",
      zero_infinity: "<class 'bool'>",
    },
  },
});

/// CosineEmbeddingLoss
/// generate by convert_torch_nn_layer.py
export const CosineEmbeddingLoss = defineLayerNode({
  node_name: "CosineEmbeddingLoss",
  params: {
    margin: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "CosineEmbeddingLoss",
    module: "torch.nn.modules.loss",
    params: {
      margin: "<class 'float'>",
      reduction: "<class 'str'>",
    },
  },
});

/// CrossEntropyLoss
/// generate by convert_torch_nn_layer.py
export const CrossEntropyLoss = defineLayerNode({
  node_name: "CrossEntropyLoss",
  params: {
    weight: zod.optional(zod.any()),
    ignore_index: zod.number(),
    reduction: zod.string(),
    label_smoothing: zod.number(),
  },
  pytorch: {
    class_name: "CrossEntropyLoss",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      ignore_index: "<class 'int'>",
      reduction: "<class 'str'>",
      label_smoothing: "<class 'float'>",
    },
  },
});

/// GaussianNLLLoss
/// generate by convert_torch_nn_layer.py
export const GaussianNLLLoss = defineLayerNode({
  node_name: "GaussianNLLLoss",
  params: {
    full: zod.boolean(),
    eps: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "GaussianNLLLoss",
    module: "torch.nn.modules.loss",
    params: {
      full: "<class 'bool'>",
      eps: "<class 'float'>",
      reduction: "<class 'str'>",
    },
  },
});

/// HingeEmbeddingLoss
/// generate by convert_torch_nn_layer.py
export const HingeEmbeddingLoss = defineLayerNode({
  node_name: "HingeEmbeddingLoss",
  params: {
    margin: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "HingeEmbeddingLoss",
    module: "torch.nn.modules.loss",
    params: {
      margin: "<class 'float'>",
      reduction: "<class 'str'>",
    },
  },
});

/// HuberLoss
/// generate by convert_torch_nn_layer.py
export const HuberLoss = defineLayerNode({
  node_name: "HuberLoss",
  params: {
    reduction: zod.string(),
    delta: zod.number(),
  },
  pytorch: {
    class_name: "HuberLoss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
      delta: "<class 'float'>",
    },
  },
});

/// KLDivLoss
/// generate by convert_torch_nn_layer.py
export const KLDivLoss = defineLayerNode({
  node_name: "KLDivLoss",
  params: {
    reduction: zod.string(),
    log_target: zod.boolean(),
  },
  pytorch: {
    class_name: "KLDivLoss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
      log_target: "<class 'bool'>",
    },
  },
});

/// L1Loss
/// generate by convert_torch_nn_layer.py
export const L1Loss = defineLayerNode({
  node_name: "L1Loss",
  params: {
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "L1Loss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
    },
  },
});

/// MSELoss
/// generate by convert_torch_nn_layer.py
export const MSELoss = defineLayerNode({
  node_name: "MSELoss",
  params: {
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "MSELoss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
    },
  },
});

/// MarginRankingLoss
/// generate by convert_torch_nn_layer.py
export const MarginRankingLoss = defineLayerNode({
  node_name: "MarginRankingLoss",
  params: {
    margin: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "MarginRankingLoss",
    module: "torch.nn.modules.loss",
    params: {
      margin: "<class 'float'>",
      reduction: "<class 'str'>",
    },
  },
});

/// MultiLabelMarginLoss
/// generate by convert_torch_nn_layer.py
export const MultiLabelMarginLoss = defineLayerNode({
  node_name: "MultiLabelMarginLoss",
  params: {
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "MultiLabelMarginLoss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
    },
  },
});

/// MultiLabelSoftMarginLoss
/// generate by convert_torch_nn_layer.py
export const MultiLabelSoftMarginLoss = defineLayerNode({
  node_name: "MultiLabelSoftMarginLoss",
  params: {
    weight: zod.optional(zod.any()),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "MultiLabelSoftMarginLoss",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      reduction: "<class 'str'>",
    },
  },
});

/// MultiMarginLoss
/// generate by convert_torch_nn_layer.py
export const MultiMarginLoss = defineLayerNode({
  node_name: "MultiMarginLoss",
  params: {
    p: zod.number(),
    margin: zod.number(),
    weight: zod.optional(zod.any()),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "MultiMarginLoss",
    module: "torch.nn.modules.loss",
    params: {
      p: "<class 'int'>",
      margin: "<class 'float'>",
      weight: "typing.Optional[torch.Tensor]",
      reduction: "<class 'str'>",
    },
  },
});

/// NLLLoss
/// generate by convert_torch_nn_layer.py
export const NLLLoss = defineLayerNode({
  node_name: "NLLLoss",
  params: {
    weight: zod.optional(zod.any()),
    ignore_index: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "NLLLoss",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      ignore_index: "<class 'int'>",
      reduction: "<class 'str'>",
    },
  },
});

/// NLLLoss2d
/// generate by convert_torch_nn_layer.py
export const NLLLoss2d = defineLayerNode({
  node_name: "NLLLoss2d",
  params: {
    weight: zod.optional(zod.any()),
    ignore_index: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "NLLLoss2d",
    module: "torch.nn.modules.loss",
    params: {
      weight: "typing.Optional[torch.Tensor]",
      ignore_index: "<class 'int'>",
      reduction: "<class 'str'>",
    },
  },
});

/// PoissonNLLLoss
/// generate by convert_torch_nn_layer.py
export const PoissonNLLLoss = defineLayerNode({
  node_name: "PoissonNLLLoss",
  params: {
    log_input: zod.boolean(),
    full: zod.boolean(),
    eps: zod.number(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "PoissonNLLLoss",
    module: "torch.nn.modules.loss",
    params: {
      log_input: "<class 'bool'>",
      full: "<class 'bool'>",
      eps: "<class 'float'>",
      reduction: "<class 'str'>",
    },
  },
});

/// SmoothL1Loss
/// generate by convert_torch_nn_layer.py
export const SmoothL1Loss = defineLayerNode({
  node_name: "SmoothL1Loss",
  params: {
    reduction: zod.string(),
    beta: zod.number(),
  },
  pytorch: {
    class_name: "SmoothL1Loss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
      beta: "<class 'float'>",
    },
  },
});

/// SoftMarginLoss
/// generate by convert_torch_nn_layer.py
export const SoftMarginLoss = defineLayerNode({
  node_name: "SoftMarginLoss",
  params: {
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "SoftMarginLoss",
    module: "torch.nn.modules.loss",
    params: {
      reduction: "<class 'str'>",
    },
  },
});

/// TripletMarginLoss
/// generate by convert_torch_nn_layer.py
export const TripletMarginLoss = defineLayerNode({
  node_name: "TripletMarginLoss",
  params: {
    margin: zod.number(),
    p: zod.number(),
    eps: zod.number(),
    swap: zod.boolean(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "TripletMarginLoss",
    module: "torch.nn.modules.loss",
    params: {
      margin: "<class 'float'>",
      p: "<class 'float'>",
      eps: "<class 'float'>",
      swap: "<class 'bool'>",
      reduction: "<class 'str'>",
    },
  },
});

/// TripletMarginWithDistanceLoss
/// generate by convert_torch_nn_layer.py
export const TripletMarginWithDistanceLoss = defineLayerNode({
  node_name: "TripletMarginWithDistanceLoss",
  params: {
    distance_function: zod.optional(zod.function()),
    margin: zod.number(),
    swap: zod.boolean(),
    reduction: zod.string(),
  },
  pytorch: {
    class_name: "TripletMarginWithDistanceLoss",
    module: "torch.nn.modules.loss",
    params: {
      distance_function:
        "typing.Optional[typing.Callable[[torch.Tensor, torch.Tensor], torch.Tensor]]",
      margin: "<class 'float'>",
      swap: "<class 'bool'>",
      reduction: "<class 'str'>",
    },
  },
});

/// Module
/// generate by convert_torch_nn_layer.py
export const Module = defineLayerNode({
  node_name: "Module",
  params: {},
  pytorch: {
    class_name: "Module",
    module: "torch.nn.modules.module",
    params: {},
  },
});

/// CrossMapLRN2d
/// generate by convert_torch_nn_layer.py
export const CrossMapLRN2d = defineLayerNode({
  node_name: "CrossMapLRN2d",
  params: {
    size: zod.number(),
    alpha: zod.number(),
    beta: zod.number(),
    k: zod.number(),
  },
  pytorch: {
    class_name: "CrossMapLRN2d",
    module: "torch.nn.modules.normalization",
    params: {
      size: "<class 'int'>",
      alpha: "<class 'float'>",
      beta: "<class 'float'>",
      k: "<class 'float'>",
    },
  },
});

/// GroupNorm
/// generate by convert_torch_nn_layer.py
export const GroupNorm = defineLayerNode({
  node_name: "GroupNorm",
  params: {
    num_groups: zod.number(),
    num_channels: zod.number(),
    eps: zod.number(),
    affine: zod.boolean(),
  },
  pytorch: {
    class_name: "GroupNorm",
    module: "torch.nn.modules.normalization",
    params: {
      num_groups: "<class 'int'>",
      num_channels: "<class 'int'>",
      eps: "<class 'float'>",
      affine: "<class 'bool'>",
    },
  },
});

/// LayerNorm
/// generate by convert_torch_nn_layer.py
export const LayerNorm = defineLayerNode({
  node_name: "LayerNorm",
  params: {
    normalized_shape: zod.union([
      zod.number(),
      zod.array(zod.number()),
      zod.any(),
    ]),
    eps: zod.number(),
    elementwise_affine: zod.boolean(),
  },
  pytorch: {
    class_name: "LayerNorm",
    module: "torch.nn.modules.normalization",
    params: {
      normalized_shape: "typing.Union[int, typing.List[int], torch.Size]",
      eps: "<class 'float'>",
      elementwise_affine: "<class 'bool'>",
    },
  },
});

/// LocalResponseNorm
/// generate by convert_torch_nn_layer.py
export const LocalResponseNorm = defineLayerNode({
  node_name: "LocalResponseNorm",
  params: {
    size: zod.number(),
    alpha: zod.number(),
    beta: zod.number(),
    k: zod.number(),
  },
  pytorch: {
    class_name: "LocalResponseNorm",
    module: "torch.nn.modules.normalization",
    params: {
      size: "<class 'int'>",
      alpha: "<class 'float'>",
      beta: "<class 'float'>",
      k: "<class 'float'>",
    },
  },
});

/// ConstantPad1d
/// generate by convert_torch_nn_layer.py
export const ConstantPad1d = defineLayerNode({
  node_name: "ConstantPad1d",
  params: {
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    value: zod.number(),
  },
  pytorch: {
    class_name: "ConstantPad1d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int]]",
      value: "<class 'float'>",
    },
  },
});

/// ConstantPad2d
/// generate by convert_torch_nn_layer.py
export const ConstantPad2d = defineLayerNode({
  node_name: "ConstantPad2d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number(), zod.number()]),
    ]),
    value: zod.number(),
  },
  pytorch: {
    class_name: "ConstantPad2d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int]]",
      value: "<class 'float'>",
    },
  },
});

/// ConstantPad3d
/// generate by convert_torch_nn_layer.py
export const ConstantPad3d = defineLayerNode({
  node_name: "ConstantPad3d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
      ]),
    ]),
    value: zod.number(),
  },
  pytorch: {
    class_name: "ConstantPad3d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int, int, int]]",
      value: "<class 'float'>",
    },
  },
});

/// ReflectionPad1d
/// generate by convert_torch_nn_layer.py
export const ReflectionPad1d = defineLayerNode({
  node_name: "ReflectionPad1d",
  params: {
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
  },
  pytorch: {
    class_name: "ReflectionPad1d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int]]",
    },
  },
});

/// ReflectionPad2d
/// generate by convert_torch_nn_layer.py
export const ReflectionPad2d = defineLayerNode({
  node_name: "ReflectionPad2d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number(), zod.number()]),
    ]),
  },
  pytorch: {
    class_name: "ReflectionPad2d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int]]",
    },
  },
});

/// ReflectionPad3d
/// generate by convert_torch_nn_layer.py
export const ReflectionPad3d = defineLayerNode({
  node_name: "ReflectionPad3d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
      ]),
    ]),
  },
  pytorch: {
    class_name: "ReflectionPad3d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int, int, int]]",
    },
  },
});

/// ReplicationPad1d
/// generate by convert_torch_nn_layer.py
export const ReplicationPad1d = defineLayerNode({
  node_name: "ReplicationPad1d",
  params: {
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
  },
  pytorch: {
    class_name: "ReplicationPad1d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int]]",
    },
  },
});

/// ReplicationPad2d
/// generate by convert_torch_nn_layer.py
export const ReplicationPad2d = defineLayerNode({
  node_name: "ReplicationPad2d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number(), zod.number()]),
    ]),
  },
  pytorch: {
    class_name: "ReplicationPad2d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int]]",
    },
  },
});

/// ReplicationPad3d
/// generate by convert_torch_nn_layer.py
export const ReplicationPad3d = defineLayerNode({
  node_name: "ReplicationPad3d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
        zod.number(),
      ]),
    ]),
  },
  pytorch: {
    class_name: "ReplicationPad3d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int, int, int]]",
    },
  },
});

/// ZeroPad2d
/// generate by convert_torch_nn_layer.py
export const ZeroPad2d = defineLayerNode({
  node_name: "ZeroPad2d",
  params: {
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number(), zod.number()]),
    ]),
  },
  pytorch: {
    class_name: "ZeroPad2d",
    module: "torch.nn.modules.padding",
    params: {
      padding: "typing.Union[int, typing.Tuple[int, int, int, int]]",
    },
  },
});

/// PixelShuffle
/// generate by convert_torch_nn_layer.py
export const PixelShuffle = defineLayerNode({
  node_name: "PixelShuffle",
  params: {
    upscale_factor: zod.number(),
  },
  pytorch: {
    class_name: "PixelShuffle",
    module: "torch.nn.modules.pixelshuffle",
    params: {
      upscale_factor: "<class 'int'>",
    },
  },
});

/// PixelUnshuffle
/// generate by convert_torch_nn_layer.py
export const PixelUnshuffle = defineLayerNode({
  node_name: "PixelUnshuffle",
  params: {
    downscale_factor: zod.number(),
  },
  pytorch: {
    class_name: "PixelUnshuffle",
    module: "torch.nn.modules.pixelshuffle",
    params: {
      downscale_factor: "<class 'int'>",
    },
  },
});

/// AdaptiveAvgPool1d
/// generate by convert_torch_nn_layer.py
export const AdaptiveAvgPool1d = defineLayerNode({
  node_name: "AdaptiveAvgPool1d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
  },
  pytorch: {
    class_name: "AdaptiveAvgPool1d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
    },
  },
});

/// AdaptiveAvgPool2d
/// generate by convert_torch_nn_layer.py
export const AdaptiveAvgPool2d = defineLayerNode({
  node_name: "AdaptiveAvgPool2d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
  },
  pytorch: {
    class_name: "AdaptiveAvgPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
    },
  },
});

/// AdaptiveAvgPool3d
/// generate by convert_torch_nn_layer.py
export const AdaptiveAvgPool3d = defineLayerNode({
  node_name: "AdaptiveAvgPool3d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
  },
  pytorch: {
    class_name: "AdaptiveAvgPool3d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
    },
  },
});

/// AdaptiveMaxPool1d
/// generate by convert_torch_nn_layer.py
export const AdaptiveMaxPool1d = defineLayerNode({
  node_name: "AdaptiveMaxPool1d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
    return_indices: zod.boolean(),
  },
  pytorch: {
    class_name: "AdaptiveMaxPool1d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
      return_indices: "<class 'bool'>",
    },
  },
});

/// AdaptiveMaxPool2d
/// generate by convert_torch_nn_layer.py
export const AdaptiveMaxPool2d = defineLayerNode({
  node_name: "AdaptiveMaxPool2d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
    return_indices: zod.boolean(),
  },
  pytorch: {
    class_name: "AdaptiveMaxPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
      return_indices: "<class 'bool'>",
    },
  },
});

/// AdaptiveMaxPool3d
/// generate by convert_torch_nn_layer.py
export const AdaptiveMaxPool3d = defineLayerNode({
  node_name: "AdaptiveMaxPool3d",
  params: {
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.optional(zod.number()), zod.any()]),
    ]).nullable(),
    return_indices: zod.boolean(),
  },
  pytorch: {
    class_name: "AdaptiveMaxPool3d",
    module: "torch.nn.modules.pooling",
    params: {
      output_size:
        "typing.Union[int, NoneType, typing.Tuple[typing.Optional[int], ...]]",
      return_indices: "<class 'bool'>",
    },
  },
});

/// AvgPool1d
/// generate by convert_torch_nn_layer.py
export const AvgPool1d = defineLayerNode({
  node_name: "AvgPool1d",
  params: {
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]),
    padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
    ceil_mode: zod.boolean(),
    count_include_pad: zod.boolean(),
  },
  pytorch: {
    class_name: "AvgPool1d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int]]",
      padding: "typing.Union[int, typing.Tuple[int]]",
      ceil_mode: "<class 'bool'>",
      count_include_pad: "<class 'bool'>",
    },
  },
});

/// AvgPool2d
/// generate by convert_torch_nn_layer.py
export const AvgPool2d = defineLayerNode({
  node_name: "AvgPool2d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])])
      .nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
    ceil_mode: zod.boolean(),
    count_include_pad: zod.boolean(),
    divisor_override: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "AvgPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, int]]",
      ceil_mode: "<class 'bool'>",
      count_include_pad: "<class 'bool'>",
      divisor_override: "typing.Optional[int]",
    },
  },
});

/// AvgPool3d
/// generate by convert_torch_nn_layer.py
export const AvgPool3d = defineLayerNode({
  node_name: "AvgPool3d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]).nullable(),
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    ceil_mode: zod.boolean(),
    count_include_pad: zod.boolean(),
    divisor_override: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "AvgPool3d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, int, int]]",
      ceil_mode: "<class 'bool'>",
      count_include_pad: "<class 'bool'>",
      divisor_override: "typing.Optional[int]",
    },
  },
});

/// FractionalMaxPool2d
/// generate by convert_torch_nn_layer.py
export const FractionalMaxPool2d = defineLayerNode({
  node_name: "FractionalMaxPool2d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]).nullable(),
    output_ratio: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]).nullable(),
    return_indices: zod.boolean(),
  },
  pytorch: {
    class_name: "FractionalMaxPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      output_size: "typing.Union[int, typing.Tuple[int, int], NoneType]",
      output_ratio: "typing.Union[float, typing.Tuple[float, float], NoneType]",
      return_indices: "<class 'bool'>",
    },
  },
});

/// FractionalMaxPool3d
/// generate by convert_torch_nn_layer.py
export const FractionalMaxPool3d = defineLayerNode({
  node_name: "FractionalMaxPool3d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    output_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]).nullable(),
    output_ratio: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]).nullable(),
    return_indices: zod.boolean(),
  },
  pytorch: {
    class_name: "FractionalMaxPool3d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      output_size: "typing.Union[int, typing.Tuple[int, int, int], NoneType]",
      output_ratio:
        "typing.Union[float, typing.Tuple[float, float, float], NoneType]",
      return_indices: "<class 'bool'>",
    },
  },
});

/// LPPool1d
/// generate by convert_torch_nn_layer.py
export const LPPool1d = defineLayerNode({
  node_name: "LPPool1d",
  params: {
    norm_type: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    ceil_mode: zod.boolean(),
  },
  pytorch: {
    class_name: "LPPool1d",
    module: "torch.nn.modules.pooling",
    params: {
      norm_type: "<class 'float'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      ceil_mode: "<class 'bool'>",
    },
  },
});

/// LPPool2d
/// generate by convert_torch_nn_layer.py
export const LPPool2d = defineLayerNode({
  node_name: "LPPool2d",
  params: {
    norm_type: zod.number(),
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    ceil_mode: zod.boolean(),
  },
  pytorch: {
    class_name: "LPPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      norm_type: "<class 'float'>",
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      ceil_mode: "<class 'bool'>",
    },
  },
});

/// MaxPool1d
/// generate by convert_torch_nn_layer.py
export const MaxPool1d = defineLayerNode({
  node_name: "MaxPool1d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    return_indices: zod.boolean(),
    ceil_mode: zod.boolean(),
  },
  pytorch: {
    class_name: "MaxPool1d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, ...]]",
      dilation: "typing.Union[int, typing.Tuple[int, ...]]",
      return_indices: "<class 'bool'>",
      ceil_mode: "<class 'bool'>",
    },
  },
});

/// MaxPool2d
/// generate by convert_torch_nn_layer.py
export const MaxPool2d = defineLayerNode({
  node_name: "MaxPool2d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    return_indices: zod.boolean(),
    ceil_mode: zod.boolean(),
  },
  pytorch: {
    class_name: "MaxPool2d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, ...]]",
      dilation: "typing.Union[int, typing.Tuple[int, ...]]",
      return_indices: "<class 'bool'>",
      ceil_mode: "<class 'bool'>",
    },
  },
});

/// MaxPool3d
/// generate by convert_torch_nn_layer.py
export const MaxPool3d = defineLayerNode({
  node_name: "MaxPool3d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    dilation: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])]),
    return_indices: zod.boolean(),
    ceil_mode: zod.boolean(),
  },
  pytorch: {
    class_name: "MaxPool3d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, ...]]",
      stride: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, ...]]",
      dilation: "typing.Union[int, typing.Tuple[int, ...]]",
      return_indices: "<class 'bool'>",
      ceil_mode: "<class 'bool'>",
    },
  },
});

/// MaxUnpool1d
/// generate by convert_torch_nn_layer.py
export const MaxUnpool1d = defineLayerNode({
  node_name: "MaxUnpool1d",
  params: {
    kernel_size: zod.union([zod.number(), zod.tuple([zod.number()])]),
    stride: zod.union([zod.number(), zod.tuple([zod.number()])]).nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number()])]),
  },
  pytorch: {
    class_name: "MaxUnpool1d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int]]",
      stride: "typing.Union[int, typing.Tuple[int], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int]]",
    },
  },
});

/// MaxUnpool2d
/// generate by convert_torch_nn_layer.py
export const MaxUnpool2d = defineLayerNode({
  node_name: "MaxUnpool2d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]),
    stride: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])])
      .nullable(),
    padding: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])]),
  },
  pytorch: {
    class_name: "MaxUnpool2d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, int]]",
    },
  },
});

/// MaxUnpool3d
/// generate by convert_torch_nn_layer.py
export const MaxUnpool3d = defineLayerNode({
  node_name: "MaxUnpool3d",
  params: {
    kernel_size: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
    stride: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]).nullable(),
    padding: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number(), zod.number()]),
    ]),
  },
  pytorch: {
    class_name: "MaxUnpool3d",
    module: "torch.nn.modules.pooling",
    params: {
      kernel_size: "typing.Union[int, typing.Tuple[int, int, int]]",
      stride: "typing.Union[int, typing.Tuple[int, int, int], NoneType]",
      padding: "typing.Union[int, typing.Tuple[int, int, int]]",
    },
  },
});

/// GRU
/// generate by convert_torch_nn_layer.py
export const GRU = defineLayerNode({
  node_name: "GRU",
  params: {},
  pytorch: {
    class_name: "GRU",
    module: "torch.nn.modules.rnn",
    params: {},
  },
});

/// GRUCell
/// generate by convert_torch_nn_layer.py
export const GRUCell = defineLayerNode({
  node_name: "GRUCell",
  params: {
    input_size: zod.number(),
    hidden_size: zod.number(),
    bias: zod.boolean(),
  },
  pytorch: {
    class_name: "GRUCell",
    module: "torch.nn.modules.rnn",
    params: {
      input_size: "<class 'int'>",
      hidden_size: "<class 'int'>",
      bias: "<class 'bool'>",
    },
  },
});

/// LSTM
/// generate by convert_torch_nn_layer.py
export const LSTM = defineLayerNode({
  node_name: "LSTM",
  params: {},
  pytorch: {
    class_name: "LSTM",
    module: "torch.nn.modules.rnn",
    params: {},
  },
});

/// LSTMCell
/// generate by convert_torch_nn_layer.py
export const LSTMCell = defineLayerNode({
  node_name: "LSTMCell",
  params: {
    input_size: zod.number(),
    hidden_size: zod.number(),
    bias: zod.boolean(),
  },
  pytorch: {
    class_name: "LSTMCell",
    module: "torch.nn.modules.rnn",
    params: {
      input_size: "<class 'int'>",
      hidden_size: "<class 'int'>",
      bias: "<class 'bool'>",
    },
  },
});

/// RNN
/// generate by convert_torch_nn_layer.py
export const RNN = defineLayerNode({
  node_name: "RNN",
  params: {},
  pytorch: {
    class_name: "RNN",
    module: "torch.nn.modules.rnn",
    params: {},
  },
});

/// RNNBase
/// generate by convert_torch_nn_layer.py
export const RNNBase = defineLayerNode({
  node_name: "RNNBase",
  params: {
    mode: zod.string(),
    input_size: zod.number(),
    hidden_size: zod.number(),
    num_layers: zod.number(),
    bias: zod.boolean(),
    batch_first: zod.boolean(),
    dropout: zod.number(),
    bidirectional: zod.boolean(),
    proj_size: zod.number(),
  },
  pytorch: {
    class_name: "RNNBase",
    module: "torch.nn.modules.rnn",
    params: {
      mode: "<class 'str'>",
      input_size: "<class 'int'>",
      hidden_size: "<class 'int'>",
      num_layers: "<class 'int'>",
      bias: "<class 'bool'>",
      batch_first: "<class 'bool'>",
      dropout: "<class 'float'>",
      bidirectional: "<class 'bool'>",
      proj_size: "<class 'int'>",
    },
  },
});

/// RNNCell
/// generate by convert_torch_nn_layer.py
export const RNNCell = defineLayerNode({
  node_name: "RNNCell",
  params: {
    input_size: zod.number(),
    hidden_size: zod.number(),
    bias: zod.boolean(),
    nonlinearity: zod.string(),
  },
  pytorch: {
    class_name: "RNNCell",
    module: "torch.nn.modules.rnn",
    params: {
      input_size: "<class 'int'>",
      hidden_size: "<class 'int'>",
      bias: "<class 'bool'>",
      nonlinearity: "<class 'str'>",
    },
  },
});

/// RNNCellBase
/// generate by convert_torch_nn_layer.py
export const RNNCellBase = defineLayerNode({
  node_name: "RNNCellBase",
  params: {
    input_size: zod.number(),
    hidden_size: zod.number(),
    bias: zod.boolean(),
    num_chunks: zod.number(),
  },
  pytorch: {
    class_name: "RNNCellBase",
    module: "torch.nn.modules.rnn",
    params: {
      input_size: "<class 'int'>",
      hidden_size: "<class 'int'>",
      bias: "<class 'bool'>",
      num_chunks: "<class 'int'>",
    },
  },
});

/// Embedding
/// generate by convert_torch_nn_layer.py
export const Embedding = defineLayerNode({
  node_name: "Embedding",
  params: {
    num_embeddings: zod.number(),
    embedding_dim: zod.number(),
    padding_idx: zod.optional(zod.number()),
    max_norm: zod.optional(zod.number()),
    norm_type: zod.number(),
    scale_grad_by_freq: zod.boolean(),
    sparse: zod.boolean(),
    _weight: zod.optional(zod.any()),
    _freeze: zod.boolean(),
  },
  pytorch: {
    class_name: "Embedding",
    module: "torch.nn.modules.sparse",
    params: {
      num_embeddings: "<class 'int'>",
      embedding_dim: "<class 'int'>",
      padding_idx: "typing.Optional[int]",
      max_norm: "typing.Optional[float]",
      norm_type: "<class 'float'>",
      scale_grad_by_freq: "<class 'bool'>",
      sparse: "<class 'bool'>",
      _weight: "typing.Optional[torch.Tensor]",
      _freeze: "<class 'bool'>",
    },
  },
});

/// EmbeddingBag
/// generate by convert_torch_nn_layer.py
export const EmbeddingBag = defineLayerNode({
  node_name: "EmbeddingBag",
  params: {
    num_embeddings: zod.number(),
    embedding_dim: zod.number(),
    max_norm: zod.optional(zod.number()),
    norm_type: zod.number(),
    scale_grad_by_freq: zod.boolean(),
    mode: zod.string(),
    sparse: zod.boolean(),
    _weight: zod.optional(zod.any()),
    include_last_offset: zod.boolean(),
    padding_idx: zod.optional(zod.number()),
  },
  pytorch: {
    class_name: "EmbeddingBag",
    module: "torch.nn.modules.sparse",
    params: {
      num_embeddings: "<class 'int'>",
      embedding_dim: "<class 'int'>",
      max_norm: "typing.Optional[float]",
      norm_type: "<class 'float'>",
      scale_grad_by_freq: "<class 'bool'>",
      mode: "<class 'str'>",
      sparse: "<class 'bool'>",
      _weight: "typing.Optional[torch.Tensor]",
      include_last_offset: "<class 'bool'>",
      padding_idx: "typing.Optional[int]",
    },
  },
});

/// Transformer
/// generate by convert_torch_nn_layer.py
export const Transformer = defineLayerNode({
  node_name: "Transformer",
  params: {
    d_model: zod.number(),
    nhead: zod.number(),
    num_encoder_layers: zod.number(),
    num_decoder_layers: zod.number(),
    dim_feedforward: zod.number(),
    dropout: zod.number(),
    activation: zod.union([zod.string(), zod.function()]),
    custom_encoder: zod.optional(zod.any()),
    custom_decoder: zod.optional(zod.any()),
    layer_norm_eps: zod.number(),
    batch_first: zod.boolean(),
    norm_first: zod.boolean(),
  },
  pytorch: {
    class_name: "Transformer",
    module: "torch.nn.modules.transformer",
    params: {
      d_model: "<class 'int'>",
      nhead: "<class 'int'>",
      num_encoder_layers: "<class 'int'>",
      num_decoder_layers: "<class 'int'>",
      dim_feedforward: "<class 'int'>",
      dropout: "<class 'float'>",
      activation:
        "typing.Union[str, typing.Callable[[torch.Tensor], torch.Tensor]]",
      custom_encoder: "typing.Optional[typing.Any]",
      custom_decoder: "typing.Optional[typing.Any]",
      layer_norm_eps: "<class 'float'>",
      batch_first: "<class 'bool'>",
      norm_first: "<class 'bool'>",
    },
  },
});

/// TransformerDecoder
/// generate by convert_torch_nn_layer.py
export const TransformerDecoder = defineLayerNode({
  node_name: "TransformerDecoder",
  params: {},
  pytorch: {
    class_name: "TransformerDecoder",
    module: "torch.nn.modules.transformer",
    params: {},
  },
});

/// TransformerDecoderLayer
/// generate by convert_torch_nn_layer.py
export const TransformerDecoderLayer = defineLayerNode({
  node_name: "TransformerDecoderLayer",
  params: {
    d_model: zod.number(),
    nhead: zod.number(),
    dim_feedforward: zod.number(),
    dropout: zod.number(),
    activation: zod.union([zod.string(), zod.function()]),
    layer_norm_eps: zod.number(),
    batch_first: zod.boolean(),
    norm_first: zod.boolean(),
  },
  pytorch: {
    class_name: "TransformerDecoderLayer",
    module: "torch.nn.modules.transformer",
    params: {
      d_model: "<class 'int'>",
      nhead: "<class 'int'>",
      dim_feedforward: "<class 'int'>",
      dropout: "<class 'float'>",
      activation:
        "typing.Union[str, typing.Callable[[torch.Tensor], torch.Tensor]]",
      layer_norm_eps: "<class 'float'>",
      batch_first: "<class 'bool'>",
      norm_first: "<class 'bool'>",
    },
  },
});

/// TransformerEncoder
/// generate by convert_torch_nn_layer.py
export const TransformerEncoder = defineLayerNode({
  node_name: "TransformerEncoder",
  params: {},
  pytorch: {
    class_name: "TransformerEncoder",
    module: "torch.nn.modules.transformer",
    params: {},
  },
});

/// TransformerEncoderLayer
/// generate by convert_torch_nn_layer.py
export const TransformerEncoderLayer = defineLayerNode({
  node_name: "TransformerEncoderLayer",
  params: {
    d_model: zod.number(),
    nhead: zod.number(),
    dim_feedforward: zod.number(),
    dropout: zod.number(),
    activation: zod.union([zod.string(), zod.function()]),
    layer_norm_eps: zod.number(),
    batch_first: zod.boolean(),
    norm_first: zod.boolean(),
  },
  pytorch: {
    class_name: "TransformerEncoderLayer",
    module: "torch.nn.modules.transformer",
    params: {
      d_model: "<class 'int'>",
      nhead: "<class 'int'>",
      dim_feedforward: "<class 'int'>",
      dropout: "<class 'float'>",
      activation:
        "typing.Union[str, typing.Callable[[torch.Tensor], torch.Tensor]]",
      layer_norm_eps: "<class 'float'>",
      batch_first: "<class 'bool'>",
      norm_first: "<class 'bool'>",
    },
  },
});

/// Upsample
/// generate by convert_torch_nn_layer.py
export const Upsample = defineLayerNode({
  node_name: "Upsample",
  params: {
    size: zod.union([zod.number(), zod.tuple([zod.number(), zod.any()])])
      .nullable(),
    scale_factor: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.any()]),
    ]).nullable(),
    mode: zod.string(),
    align_corners: zod.optional(zod.boolean()),
    recompute_scale_factor: zod.optional(zod.boolean()),
  },
  pytorch: {
    class_name: "Upsample",
    module: "torch.nn.modules.upsampling",
    params: {
      size: "typing.Union[int, typing.Tuple[int, ...], NoneType]",
      scale_factor: "typing.Union[float, typing.Tuple[float, ...], NoneType]",
      mode: "<class 'str'>",
      align_corners: "typing.Optional[bool]",
      recompute_scale_factor: "typing.Optional[bool]",
    },
  },
});

/// UpsamplingBilinear2d
/// generate by convert_torch_nn_layer.py
export const UpsamplingBilinear2d = defineLayerNode({
  node_name: "UpsamplingBilinear2d",
  params: {
    size: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])])
      .nullable(),
    scale_factor: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]).nullable(),
  },
  pytorch: {
    class_name: "UpsamplingBilinear2d",
    module: "torch.nn.modules.upsampling",
    params: {
      size: "typing.Union[int, typing.Tuple[int, int], NoneType]",
      scale_factor: "typing.Union[float, typing.Tuple[float, float], NoneType]",
    },
  },
});

/// UpsamplingNearest2d
/// generate by convert_torch_nn_layer.py
export const UpsamplingNearest2d = defineLayerNode({
  node_name: "UpsamplingNearest2d",
  params: {
    size: zod.union([zod.number(), zod.tuple([zod.number(), zod.number()])])
      .nullable(),
    scale_factor: zod.union([
      zod.number(),
      zod.tuple([zod.number(), zod.number()]),
    ]).nullable(),
  },
  pytorch: {
    class_name: "UpsamplingNearest2d",
    module: "torch.nn.modules.upsampling",
    params: {
      size: "typing.Union[int, typing.Tuple[int, int], NoneType]",
      scale_factor: "typing.Union[float, typing.Tuple[float, float], NoneType]",
    },
  },
});
