import { defineLayerNode } from './base';import * as zod from 'zod';
/// TFSMLayer
/// generate by convert_tfk_layer.py
export const TFSMLayer =  defineLayerNode({
    node_name: "TFSMLayer",
    params: {

        },
    pytorch: {
        class_name: "TFSMLayer",
        module: "keras.src.export.export_lib",
        params: {
filepath : "serve",
call_endpoint : "None",
call_training_endpoint : "True",
trainable : "None",
dtype : "None",
        },
    },
});
    
/// Activation
/// generate by convert_tfk_layer.py
export const Activation =  defineLayerNode({
    node_name: "Activation",
    params: {

        },
    pytorch: {
        class_name: "Activation",
        module: "keras.src.layers.activations.activation",
        params: {
activation : "NoneType",
        },
    },
});
    
/// ELU
/// generate by convert_tfk_layer.py
export const ELU =  defineLayerNode({
    node_name: "ELU",
    params: {

        },
    pytorch: {
        class_name: "ELU",
        module: "keras.src.layers.activations.elu",
        params: {
alpha : "1.0",
        },
    },
});
    
/// LeakyReLU
/// generate by convert_tfk_layer.py
export const LeakyReLU =  defineLayerNode({
    node_name: "LeakyReLU",
    params: {

        },
    pytorch: {
        class_name: "LeakyReLU",
        module: "keras.src.layers.activations.leaky_relu",
        params: {
negative_slope : "0.3",
        },
    },
});
    
/// PReLU
/// generate by convert_tfk_layer.py
export const PReLU =  defineLayerNode({
    node_name: "PReLU",
    params: {

        },
    pytorch: {
        class_name: "PReLU",
        module: "keras.src.layers.activations.prelu",
        params: {
alpha_initializer : "Zeros",
alpha_regularizer : "None",
alpha_constraint : "None",
shared_axes : "None",
        },
    },
});
    
/// ReLU
/// generate by convert_tfk_layer.py
export const ReLU =  defineLayerNode({
    node_name: "ReLU",
    params: {

        },
    pytorch: {
        class_name: "ReLU",
        module: "keras.src.layers.activations.relu",
        params: {
max_value : "None",
negative_slope : "0.0",
threshold : "0.0",
        },
    },
});
    
/// Softmax
/// generate by convert_tfk_layer.py
export const Softmax =  defineLayerNode({
    node_name: "Softmax",
    params: {

        },
    pytorch: {
        class_name: "Softmax",
        module: "keras.src.layers.activations.softmax",
        params: {
axis : "-1",
        },
    },
});
    
/// AdditiveAttention
/// generate by convert_tfk_layer.py
export const AdditiveAttention =  defineLayerNode({
    node_name: "AdditiveAttention",
    params: {

        },
    pytorch: {
        class_name: "AdditiveAttention",
        module: "keras.src.layers.attention.additive_attention",
        params: {
use_scale : "True",
dropout : "0.0",
        },
    },
});
    
/// Attention
/// generate by convert_tfk_layer.py
export const Attention =  defineLayerNode({
    node_name: "Attention",
    params: {

        },
    pytorch: {
        class_name: "Attention",
        module: "keras.src.layers.attention.attention",
        params: {
use_scale : "False",
score_mode : "dot",
dropout : "0.0",
seed : "None",
        },
    },
});
    
/// GroupedQueryAttention
/// generate by convert_tfk_layer.py
export const GroupedQueryAttention =  defineLayerNode({
    node_name: "GroupedQueryAttention",
    params: {

        },
    pytorch: {
        class_name: "GroupedQueryAttention",
        module: "keras.src.layers.attention.grouped_query_attention",
        params: {
head_dim : "0.0",
num_query_heads : "True",
num_key_value_heads : "glorot_uniform",
dropout : "zeros",
use_bias : "None",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "NoneType",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// MultiHeadAttention
/// generate by convert_tfk_layer.py
export const MultiHeadAttention =  defineLayerNode({
    node_name: "MultiHeadAttention",
    params: {

        },
    pytorch: {
        class_name: "MultiHeadAttention",
        module: "keras.src.layers.attention.multi_head_attention",
        params: {
num_heads : "None",
key_dim : "0.0",
value_dim : "True",
dropout : "None",
use_bias : "None",
output_shape : "glorot_uniform",
attention_axes : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv1D
/// generate by convert_tfk_layer.py
export const Conv1D =  defineLayerNode({
    node_name: "Conv1D",
    params: {

        },
    pytorch: {
        class_name: "Conv1D",
        module: "keras.src.layers.convolutional.conv1d",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "1",
dilation_rate : "None",
groups : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv1DTranspose
/// generate by convert_tfk_layer.py
export const Conv1DTranspose =  defineLayerNode({
    node_name: "Conv1DTranspose",
    params: {

        },
    pytorch: {
        class_name: "Conv1DTranspose",
        module: "keras.src.layers.convolutional.conv1d_transpose",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "None",
dilation_rate : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv2D
/// generate by convert_tfk_layer.py
export const Conv2D =  defineLayerNode({
    node_name: "Conv2D",
    params: {

        },
    pytorch: {
        class_name: "Conv2D",
        module: "keras.src.layers.convolutional.conv2d",
        params: {
filters : "(1, 1)",
kernel_size : "valid",
strides : "None",
padding : "(1, 1)",
data_format : "1",
dilation_rate : "None",
groups : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv2DTranspose
/// generate by convert_tfk_layer.py
export const Conv2DTranspose =  defineLayerNode({
    node_name: "Conv2DTranspose",
    params: {

        },
    pytorch: {
        class_name: "Conv2DTranspose",
        module: "keras.src.layers.convolutional.conv2d_transpose",
        params: {
filters : "(1, 1)",
kernel_size : "valid",
strides : "None",
padding : "(1, 1)",
data_format : "None",
dilation_rate : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv3D
/// generate by convert_tfk_layer.py
export const Conv3D =  defineLayerNode({
    node_name: "Conv3D",
    params: {

        },
    pytorch: {
        class_name: "Conv3D",
        module: "keras.src.layers.convolutional.conv3d",
        params: {
filters : "(1, 1, 1)",
kernel_size : "valid",
strides : "None",
padding : "(1, 1, 1)",
data_format : "1",
dilation_rate : "None",
groups : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Conv3DTranspose
/// generate by convert_tfk_layer.py
export const Conv3DTranspose =  defineLayerNode({
    node_name: "Conv3DTranspose",
    params: {

        },
    pytorch: {
        class_name: "Conv3DTranspose",
        module: "keras.src.layers.convolutional.conv3d_transpose",
        params: {
filters : "(1, 1, 1)",
kernel_size : "valid",
strides : "None",
padding : "(1, 1, 1)",
data_format : "None",
dilation_rate : "True",
activation : "glorot_uniform",
use_bias : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// DepthwiseConv1D
/// generate by convert_tfk_layer.py
export const DepthwiseConv1D =  defineLayerNode({
    node_name: "DepthwiseConv1D",
    params: {

        },
    pytorch: {
        class_name: "DepthwiseConv1D",
        module: "keras.src.layers.convolutional.depthwise_conv1d",
        params: {
kernel_size : "1",
strides : "valid",
padding : "1",
depth_multiplier : "None",
data_format : "1",
dilation_rate : "None",
activation : "True",
use_bias : "glorot_uniform",
depthwise_initializer : "zeros",
bias_initializer : "None",
depthwise_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
depthwise_constraint : "None",
bias_constraint : "NoneType",
        },
    },
});
    
/// DepthwiseConv2D
/// generate by convert_tfk_layer.py
export const DepthwiseConv2D =  defineLayerNode({
    node_name: "DepthwiseConv2D",
    params: {

        },
    pytorch: {
        class_name: "DepthwiseConv2D",
        module: "keras.src.layers.convolutional.depthwise_conv2d",
        params: {
kernel_size : "(1, 1)",
strides : "valid",
padding : "1",
depth_multiplier : "None",
data_format : "(1, 1)",
dilation_rate : "None",
activation : "True",
use_bias : "glorot_uniform",
depthwise_initializer : "zeros",
bias_initializer : "None",
depthwise_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
depthwise_constraint : "None",
bias_constraint : "NoneType",
        },
    },
});
    
/// SeparableConv1D
/// generate by convert_tfk_layer.py
export const SeparableConv1D =  defineLayerNode({
    node_name: "SeparableConv1D",
    params: {

        },
    pytorch: {
        class_name: "SeparableConv1D",
        module: "keras.src.layers.convolutional.separable_conv1d",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "1",
dilation_rate : "None",
depth_multiplier : "True",
activation : "glorot_uniform",
use_bias : "glorot_uniform",
depthwise_initializer : "zeros",
pointwise_initializer : "None",
bias_initializer : "None",
depthwise_regularizer : "None",
pointwise_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
depthwise_constraint : "None",
pointwise_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// SeparableConv2D
/// generate by convert_tfk_layer.py
export const SeparableConv2D =  defineLayerNode({
    node_name: "SeparableConv2D",
    params: {

        },
    pytorch: {
        class_name: "SeparableConv2D",
        module: "keras.src.layers.convolutional.separable_conv2d",
        params: {
filters : "(1, 1)",
kernel_size : "valid",
strides : "None",
padding : "(1, 1)",
data_format : "1",
dilation_rate : "None",
depth_multiplier : "True",
activation : "glorot_uniform",
use_bias : "glorot_uniform",
depthwise_initializer : "zeros",
pointwise_initializer : "None",
bias_initializer : "None",
depthwise_regularizer : "None",
pointwise_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
depthwise_constraint : "None",
pointwise_constraint : "NoneType",
bias_constraint : "NoneType",
        },
    },
});
    
/// Dense
/// generate by convert_tfk_layer.py
export const Dense =  defineLayerNode({
    node_name: "Dense",
    params: {

        },
    pytorch: {
        class_name: "Dense",
        module: "keras.src.layers.core.dense",
        params: {
units : "None",
activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "zeros",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
bias_constraint : "None",
lora_rank : "NoneType",
        },
    },
});
    
/// EinsumDense
/// generate by convert_tfk_layer.py
export const EinsumDense =  defineLayerNode({
    node_name: "EinsumDense",
    params: {

        },
    pytorch: {
        class_name: "EinsumDense",
        module: "keras.src.layers.core.einsum_dense",
        params: {
equation : "None",
output_shape : "None",
activation : "glorot_uniform",
bias_axes : "zeros",
kernel_initializer : "None",
bias_initializer : "None",
kernel_regularizer : "None",
bias_regularizer : "None",
kernel_constraint : "None",
bias_constraint : "NoneType",
lora_rank : "NoneType",
        },
    },
});
    
/// Embedding
/// generate by convert_tfk_layer.py
export const Embedding =  defineLayerNode({
    node_name: "Embedding",
    params: {

        },
    pytorch: {
        class_name: "Embedding",
        module: "keras.src.layers.core.embedding",
        params: {
input_dim : "uniform",
output_dim : "None",
embeddings_initializer : "None",
embeddings_regularizer : "False",
embeddings_constraint : "None",
mask_zero : "None",
weights : "NoneType",
lora_rank : "NoneType",
        },
    },
});
    
/// Identity
/// generate by convert_tfk_layer.py
export const Identity =  defineLayerNode({
    node_name: "Identity",
    params: {

        },
    pytorch: {
        class_name: "Identity",
        module: "keras.src.layers.core.identity",
        params: {

        },
    },
});
    
/// InputLayer
/// generate by convert_tfk_layer.py
export const InputLayer =  defineLayerNode({
    node_name: "InputLayer",
    params: {

        },
    pytorch: {
        class_name: "InputLayer",
        module: "keras.src.layers.core.input_layer",
        params: {
shape : "None",
batch_size : "None",
dtype : "None",
sparse : "None",
batch_shape : "None",
input_tensor : "None",
        },
    },
});
    
/// Lambda
/// generate by convert_tfk_layer.py
export const Lambda =  defineLayerNode({
    node_name: "Lambda",
    params: {

        },
    pytorch: {
        class_name: "Lambda",
        module: "keras.src.layers.core.lambda_layer",
        params: {
function : "None",
output_shape : "None",
mask : "None",
arguments : "NoneType",
        },
    },
});
    
/// Masking
/// generate by convert_tfk_layer.py
export const Masking =  defineLayerNode({
    node_name: "Masking",
    params: {

        },
    pytorch: {
        class_name: "Masking",
        module: "keras.src.layers.core.masking",
        params: {
mask_value : "0.0",
        },
    },
});
    
/// Wrapper
/// generate by convert_tfk_layer.py
export const Wrapper =  defineLayerNode({
    node_name: "Wrapper",
    params: {

        },
    pytorch: {
        class_name: "Wrapper",
        module: "keras.src.layers.core.wrapper",
        params: {
layer : "NoneType",
        },
    },
});
    
/// InputSpec
/// generate by convert_tfk_layer.py
export const InputSpec =  defineLayerNode({
    node_name: "InputSpec",
    params: {

        },
    pytorch: {
        class_name: "InputSpec",
        module: "keras.src.layers.input_spec",
        params: {
dtype : "None",
shape : "None",
ndim : "None",
max_ndim : "None",
min_ndim : "None",
axes : "None",
allow_last_axis_squeeze : "False",
        },
    },
});
    
/// Layer
/// generate by convert_tfk_layer.py
export const Layer =  defineLayerNode({
    node_name: "Layer",
    params: {

        },
    pytorch: {
        class_name: "Layer",
        module: "keras.src.layers.layer",
        params: {

        },
    },
});
    
/// Add
/// generate by convert_tfk_layer.py
export const Add =  defineLayerNode({
    node_name: "Add",
    params: {

        },
    pytorch: {
        class_name: "Add",
        module: "keras.src.layers.merging.add",
        params: {

        },
    },
});
    
/// Average
/// generate by convert_tfk_layer.py
export const Average =  defineLayerNode({
    node_name: "Average",
    params: {

        },
    pytorch: {
        class_name: "Average",
        module: "keras.src.layers.merging.average",
        params: {

        },
    },
});
    
/// Concatenate
/// generate by convert_tfk_layer.py
export const Concatenate =  defineLayerNode({
    node_name: "Concatenate",
    params: {

        },
    pytorch: {
        class_name: "Concatenate",
        module: "keras.src.layers.merging.concatenate",
        params: {
axis : "-1",
        },
    },
});
    
/// Dot
/// generate by convert_tfk_layer.py
export const Dot =  defineLayerNode({
    node_name: "Dot",
    params: {

        },
    pytorch: {
        class_name: "Dot",
        module: "keras.src.layers.merging.dot",
        params: {
axes : "False",
normalize : "NoneType",
        },
    },
});
    
/// Maximum
/// generate by convert_tfk_layer.py
export const Maximum =  defineLayerNode({
    node_name: "Maximum",
    params: {

        },
    pytorch: {
        class_name: "Maximum",
        module: "keras.src.layers.merging.maximum",
        params: {

        },
    },
});
    
/// Minimum
/// generate by convert_tfk_layer.py
export const Minimum =  defineLayerNode({
    node_name: "Minimum",
    params: {

        },
    pytorch: {
        class_name: "Minimum",
        module: "keras.src.layers.merging.minimum",
        params: {

        },
    },
});
    
/// Multiply
/// generate by convert_tfk_layer.py
export const Multiply =  defineLayerNode({
    node_name: "Multiply",
    params: {

        },
    pytorch: {
        class_name: "Multiply",
        module: "keras.src.layers.merging.multiply",
        params: {

        },
    },
});
    
/// Subtract
/// generate by convert_tfk_layer.py
export const Subtract =  defineLayerNode({
    node_name: "Subtract",
    params: {

        },
    pytorch: {
        class_name: "Subtract",
        module: "keras.src.layers.merging.subtract",
        params: {

        },
    },
});
    
/// BatchNormalization
/// generate by convert_tfk_layer.py
export const BatchNormalization =  defineLayerNode({
    node_name: "BatchNormalization",
    params: {

        },
    pytorch: {
        class_name: "BatchNormalization",
        module: "keras.src.layers.normalization.batch_normalization",
        params: {
axis : "-1",
momentum : "0.99",
epsilon : "0.001",
center : "True",
scale : "True",
beta_initializer : "zeros",
gamma_initializer : "ones",
moving_mean_initializer : "zeros",
moving_variance_initializer : "ones",
beta_regularizer : "None",
gamma_regularizer : "None",
beta_constraint : "None",
gamma_constraint : "None",
synchronized : "False",
        },
    },
});
    
/// GroupNormalization
/// generate by convert_tfk_layer.py
export const GroupNormalization =  defineLayerNode({
    node_name: "GroupNormalization",
    params: {

        },
    pytorch: {
        class_name: "GroupNormalization",
        module: "keras.src.layers.normalization.group_normalization",
        params: {
groups : "32",
axis : "-1",
epsilon : "0.001",
center : "True",
scale : "True",
beta_initializer : "zeros",
gamma_initializer : "ones",
beta_regularizer : "None",
gamma_regularizer : "None",
beta_constraint : "None",
gamma_constraint : "None",
        },
    },
});
    
/// LayerNormalization
/// generate by convert_tfk_layer.py
export const LayerNormalization =  defineLayerNode({
    node_name: "LayerNormalization",
    params: {

        },
    pytorch: {
        class_name: "LayerNormalization",
        module: "keras.src.layers.normalization.layer_normalization",
        params: {
axis : "-1",
epsilon : "0.001",
center : "True",
scale : "True",
rms_scaling : "False",
beta_initializer : "zeros",
gamma_initializer : "ones",
beta_regularizer : "None",
gamma_regularizer : "None",
beta_constraint : "None",
gamma_constraint : "None",
        },
    },
});
    
/// SpectralNormalization
/// generate by convert_tfk_layer.py
export const SpectralNormalization =  defineLayerNode({
    node_name: "SpectralNormalization",
    params: {

        },
    pytorch: {
        class_name: "SpectralNormalization",
        module: "keras.src.layers.normalization.spectral_normalization",
        params: {
layer : "1",
power_iterations : "NoneType",
        },
    },
});
    
/// UnitNormalization
/// generate by convert_tfk_layer.py
export const UnitNormalization =  defineLayerNode({
    node_name: "UnitNormalization",
    params: {

        },
    pytorch: {
        class_name: "UnitNormalization",
        module: "keras.src.layers.normalization.unit_normalization",
        params: {
axis : "-1",
        },
    },
});
    
/// AveragePooling1D
/// generate by convert_tfk_layer.py
export const AveragePooling1D =  defineLayerNode({
    node_name: "AveragePooling1D",
    params: {

        },
    pytorch: {
        class_name: "AveragePooling1D",
        module: "keras.src.layers.pooling.average_pooling1d",
        params: {
pool_size : "None",
strides : "valid",
padding : "None",
data_format : "None",
        },
    },
});
    
/// AveragePooling2D
/// generate by convert_tfk_layer.py
export const AveragePooling2D =  defineLayerNode({
    node_name: "AveragePooling2D",
    params: {

        },
    pytorch: {
        class_name: "AveragePooling2D",
        module: "keras.src.layers.pooling.average_pooling2d",
        params: {
pool_size : "None",
strides : "valid",
padding : "None",
data_format : "None",
        },
    },
});
    
/// AveragePooling3D
/// generate by convert_tfk_layer.py
export const AveragePooling3D =  defineLayerNode({
    node_name: "AveragePooling3D",
    params: {

        },
    pytorch: {
        class_name: "AveragePooling3D",
        module: "keras.src.layers.pooling.average_pooling3d",
        params: {
pool_size : "None",
strides : "valid",
padding : "None",
data_format : "None",
        },
    },
});
    
/// GlobalAveragePooling1D
/// generate by convert_tfk_layer.py
export const GlobalAveragePooling1D =  defineLayerNode({
    node_name: "GlobalAveragePooling1D",
    params: {

        },
    pytorch: {
        class_name: "GlobalAveragePooling1D",
        module: "keras.src.layers.pooling.global_average_pooling1d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// GlobalAveragePooling2D
/// generate by convert_tfk_layer.py
export const GlobalAveragePooling2D =  defineLayerNode({
    node_name: "GlobalAveragePooling2D",
    params: {

        },
    pytorch: {
        class_name: "GlobalAveragePooling2D",
        module: "keras.src.layers.pooling.global_average_pooling2d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// GlobalAveragePooling3D
/// generate by convert_tfk_layer.py
export const GlobalAveragePooling3D =  defineLayerNode({
    node_name: "GlobalAveragePooling3D",
    params: {

        },
    pytorch: {
        class_name: "GlobalAveragePooling3D",
        module: "keras.src.layers.pooling.global_average_pooling3d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// GlobalMaxPooling1D
/// generate by convert_tfk_layer.py
export const GlobalMaxPooling1D =  defineLayerNode({
    node_name: "GlobalMaxPooling1D",
    params: {

        },
    pytorch: {
        class_name: "GlobalMaxPooling1D",
        module: "keras.src.layers.pooling.global_max_pooling1d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// GlobalMaxPooling2D
/// generate by convert_tfk_layer.py
export const GlobalMaxPooling2D =  defineLayerNode({
    node_name: "GlobalMaxPooling2D",
    params: {

        },
    pytorch: {
        class_name: "GlobalMaxPooling2D",
        module: "keras.src.layers.pooling.global_max_pooling2d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// GlobalMaxPooling3D
/// generate by convert_tfk_layer.py
export const GlobalMaxPooling3D =  defineLayerNode({
    node_name: "GlobalMaxPooling3D",
    params: {

        },
    pytorch: {
        class_name: "GlobalMaxPooling3D",
        module: "keras.src.layers.pooling.global_max_pooling3d",
        params: {
data_format : "None",
keepdims : "False",
        },
    },
});
    
/// MaxPooling1D
/// generate by convert_tfk_layer.py
export const MaxPooling1D =  defineLayerNode({
    node_name: "MaxPooling1D",
    params: {

        },
    pytorch: {
        class_name: "MaxPooling1D",
        module: "keras.src.layers.pooling.max_pooling1d",
        params: {
pool_size : "2",
strides : "None",
padding : "valid",
data_format : "None",
        },
    },
});
    
/// MaxPooling2D
/// generate by convert_tfk_layer.py
export const MaxPooling2D =  defineLayerNode({
    node_name: "MaxPooling2D",
    params: {

        },
    pytorch: {
        class_name: "MaxPooling2D",
        module: "keras.src.layers.pooling.max_pooling2d",
        params: {
pool_size : "(2, 2)",
strides : "None",
padding : "valid",
data_format : "None",
        },
    },
});
    
/// MaxPooling3D
/// generate by convert_tfk_layer.py
export const MaxPooling3D =  defineLayerNode({
    node_name: "MaxPooling3D",
    params: {

        },
    pytorch: {
        class_name: "MaxPooling3D",
        module: "keras.src.layers.pooling.max_pooling3d",
        params: {
pool_size : "(2, 2, 2)",
strides : "None",
padding : "valid",
data_format : "None",
        },
    },
});
    
/// MelSpectrogram
/// generate by convert_tfk_layer.py
export const MelSpectrogram =  defineLayerNode({
    node_name: "MelSpectrogram",
    params: {

        },
    pytorch: {
        class_name: "MelSpectrogram",
        module: "keras.src.layers.preprocessing.audio_preprocessing",
        params: {
fft_length : "2048",
sequence_stride : "512",
sequence_length : "None",
window : "hann",
sampling_rate : "16000",
num_mel_bins : "128",
min_freq : "20.0",
max_freq : "None",
power_to_db : "True",
top_db : "80.0",
mag_exp : "2.0",
min_power : "1e-10",
ref_power : "1.0",
        },
    },
});
    
/// CategoryEncoding
/// generate by convert_tfk_layer.py
export const CategoryEncoding =  defineLayerNode({
    node_name: "CategoryEncoding",
    params: {

        },
    pytorch: {
        class_name: "CategoryEncoding",
        module: "keras.src.layers.preprocessing.category_encoding",
        params: {
num_tokens : "None",
output_mode : "multi_hot",
sparse : "False",
        },
    },
});
    
/// CenterCrop
/// generate by convert_tfk_layer.py
export const CenterCrop =  defineLayerNode({
    node_name: "CenterCrop",
    params: {

        },
    pytorch: {
        class_name: "CenterCrop",
        module: "keras.src.layers.preprocessing.center_crop",
        params: {
height : "None",
width : "NoneType",
data_format : "NoneType",
        },
    },
});
    
/// Discretization
/// generate by convert_tfk_layer.py
export const Discretization =  defineLayerNode({
    node_name: "Discretization",
    params: {

        },
    pytorch: {
        class_name: "Discretization",
        module: "keras.src.layers.preprocessing.discretization",
        params: {
bin_boundaries : "None",
num_bins : "None",
epsilon : "0.01",
output_mode : "int",
sparse : "False",
dtype : "None",
        },
    },
});
    
/// HashedCrossing
/// generate by convert_tfk_layer.py
export const HashedCrossing =  defineLayerNode({
    node_name: "HashedCrossing",
    params: {

        },
    pytorch: {
        class_name: "HashedCrossing",
        module: "keras.src.layers.preprocessing.hashed_crossing",
        params: {
num_bins : "int",
output_mode : "False",
sparse : "None",
dtype : "None",
        },
    },
});
    
/// Hashing
/// generate by convert_tfk_layer.py
export const Hashing =  defineLayerNode({
    node_name: "Hashing",
    params: {

        },
    pytorch: {
        class_name: "Hashing",
        module: "keras.src.layers.preprocessing.hashing",
        params: {
num_bins : "None",
mask_value : "None",
salt : "int",
output_mode : "False",
sparse : "NoneType",
        },
    },
});
    
/// IntegerLookup
/// generate by convert_tfk_layer.py
export const IntegerLookup =  defineLayerNode({
    node_name: "IntegerLookup",
    params: {

        },
    pytorch: {
        class_name: "IntegerLookup",
        module: "keras.src.layers.preprocessing.integer_lookup",
        params: {
max_tokens : "None",
num_oov_indices : "1",
mask_token : "None",
oov_token : "-1",
vocabulary : "None",
vocabulary_dtype : "int64",
idf_weights : "None",
invert : "False",
output_mode : "int",
sparse : "False",
pad_to_max_tokens : "False",
        },
    },
});
    
/// Normalization
/// generate by convert_tfk_layer.py
export const Normalization =  defineLayerNode({
    node_name: "Normalization",
    params: {

        },
    pytorch: {
        class_name: "Normalization",
        module: "keras.src.layers.preprocessing.normalization",
        params: {
axis : "-1",
mean : "None",
variance : "None",
invert : "False",
        },
    },
});
    
/// RandomBrightness
/// generate by convert_tfk_layer.py
export const RandomBrightness =  defineLayerNode({
    node_name: "RandomBrightness",
    params: {

        },
    pytorch: {
        class_name: "RandomBrightness",
        module: "keras.src.layers.preprocessing.random_brightness",
        params: {
factor : "(0, 255)",
value_range : "None",
seed : "NoneType",
        },
    },
});
    
/// RandomContrast
/// generate by convert_tfk_layer.py
export const RandomContrast =  defineLayerNode({
    node_name: "RandomContrast",
    params: {

        },
    pytorch: {
        class_name: "RandomContrast",
        module: "keras.src.layers.preprocessing.random_contrast",
        params: {
factor : "None",
seed : "NoneType",
        },
    },
});
    
/// RandomCrop
/// generate by convert_tfk_layer.py
export const RandomCrop =  defineLayerNode({
    node_name: "RandomCrop",
    params: {

        },
    pytorch: {
        class_name: "RandomCrop",
        module: "keras.src.layers.preprocessing.random_crop",
        params: {
height : "None",
width : "None",
seed : "None",
data_format : "NoneType",
        },
    },
});
    
/// RandomFlip
/// generate by convert_tfk_layer.py
export const RandomFlip =  defineLayerNode({
    node_name: "RandomFlip",
    params: {

        },
    pytorch: {
        class_name: "RandomFlip",
        module: "keras.src.layers.preprocessing.random_flip",
        params: {
mode : "horizontal_and_vertical",
seed : "None",
        },
    },
});
    
/// RandomRotation
/// generate by convert_tfk_layer.py
export const RandomRotation =  defineLayerNode({
    node_name: "RandomRotation",
    params: {

        },
    pytorch: {
        class_name: "RandomRotation",
        module: "keras.src.layers.preprocessing.random_rotation",
        params: {
factor : "reflect",
fill_mode : "bilinear",
interpolation : "None",
seed : "0.0",
fill_value : "(0, 255)",
value_range : "None",
data_format : "NoneType",
        },
    },
});
    
/// RandomTranslation
/// generate by convert_tfk_layer.py
export const RandomTranslation =  defineLayerNode({
    node_name: "RandomTranslation",
    params: {

        },
    pytorch: {
        class_name: "RandomTranslation",
        module: "keras.src.layers.preprocessing.random_translation",
        params: {
height_factor : "reflect",
width_factor : "bilinear",
fill_mode : "None",
interpolation : "0.0",
seed : "None",
fill_value : "NoneType",
data_format : "NoneType",
        },
    },
});
    
/// RandomZoom
/// generate by convert_tfk_layer.py
export const RandomZoom =  defineLayerNode({
    node_name: "RandomZoom",
    params: {

        },
    pytorch: {
        class_name: "RandomZoom",
        module: "keras.src.layers.preprocessing.random_zoom",
        params: {
height_factor : "None",
width_factor : "reflect",
fill_mode : "bilinear",
interpolation : "None",
seed : "0.0",
fill_value : "None",
data_format : "NoneType",
        },
    },
});
    
/// Rescaling
/// generate by convert_tfk_layer.py
export const Rescaling =  defineLayerNode({
    node_name: "Rescaling",
    params: {

        },
    pytorch: {
        class_name: "Rescaling",
        module: "keras.src.layers.preprocessing.rescaling",
        params: {
scale : "0.0",
offset : "NoneType",
        },
    },
});
    
/// Resizing
/// generate by convert_tfk_layer.py
export const Resizing =  defineLayerNode({
    node_name: "Resizing",
    params: {

        },
    pytorch: {
        class_name: "Resizing",
        module: "keras.src.layers.preprocessing.resizing",
        params: {
height : "bilinear",
width : "False",
interpolation : "False",
crop_to_aspect_ratio : "constant",
pad_to_aspect_ratio : "0.0",
fill_mode : "None",
fill_value : "NoneType",
data_format : "NoneType",
        },
    },
});
    
/// StringLookup
/// generate by convert_tfk_layer.py
export const StringLookup =  defineLayerNode({
    node_name: "StringLookup",
    params: {

        },
    pytorch: {
        class_name: "StringLookup",
        module: "keras.src.layers.preprocessing.string_lookup",
        params: {
max_tokens : "None",
num_oov_indices : "1",
mask_token : "None",
oov_token : "[UNK]",
vocabulary : "None",
idf_weights : "None",
invert : "False",
output_mode : "int",
pad_to_max_tokens : "False",
sparse : "False",
encoding : "utf-8",
        },
    },
});
    
/// TextVectorization
/// generate by convert_tfk_layer.py
export const TextVectorization =  defineLayerNode({
    node_name: "TextVectorization",
    params: {

        },
    pytorch: {
        class_name: "TextVectorization",
        module: "keras.src.layers.preprocessing.text_vectorization",
        params: {
max_tokens : "None",
standardize : "lower_and_strip_punctuation",
split : "whitespace",
ngrams : "None",
output_mode : "int",
output_sequence_length : "None",
pad_to_max_tokens : "False",
vocabulary : "None",
idf_weights : "None",
sparse : "False",
ragged : "False",
encoding : "utf-8",
        },
    },
});
    
/// ActivityRegularization
/// generate by convert_tfk_layer.py
export const ActivityRegularization =  defineLayerNode({
    node_name: "ActivityRegularization",
    params: {

        },
    pytorch: {
        class_name: "ActivityRegularization",
        module: "keras.src.layers.regularization.activity_regularization",
        params: {
l1 : "0.0",
l2 : "0.0",
        },
    },
});
    
/// Dropout
/// generate by convert_tfk_layer.py
export const Dropout =  defineLayerNode({
    node_name: "Dropout",
    params: {

        },
    pytorch: {
        class_name: "Dropout",
        module: "keras.src.layers.regularization.dropout",
        params: {
rate : "None",
noise_shape : "None",
seed : "NoneType",
        },
    },
});
    
/// GaussianDropout
/// generate by convert_tfk_layer.py
export const GaussianDropout =  defineLayerNode({
    node_name: "GaussianDropout",
    params: {

        },
    pytorch: {
        class_name: "GaussianDropout",
        module: "keras.src.layers.regularization.gaussian_dropout",
        params: {
rate : "None",
seed : "NoneType",
        },
    },
});
    
/// GaussianNoise
/// generate by convert_tfk_layer.py
export const GaussianNoise =  defineLayerNode({
    node_name: "GaussianNoise",
    params: {

        },
    pytorch: {
        class_name: "GaussianNoise",
        module: "keras.src.layers.regularization.gaussian_noise",
        params: {
stddev : "None",
seed : "NoneType",
        },
    },
});
    
/// SpatialDropout1D
/// generate by convert_tfk_layer.py
export const SpatialDropout1D =  defineLayerNode({
    node_name: "SpatialDropout1D",
    params: {

        },
    pytorch: {
        class_name: "SpatialDropout1D",
        module: "keras.src.layers.regularization.spatial_dropout",
        params: {
rate : "None",
seed : "None",
dtype : "None",
        },
    },
});
    
/// SpatialDropout2D
/// generate by convert_tfk_layer.py
export const SpatialDropout2D =  defineLayerNode({
    node_name: "SpatialDropout2D",
    params: {

        },
    pytorch: {
        class_name: "SpatialDropout2D",
        module: "keras.src.layers.regularization.spatial_dropout",
        params: {
rate : "None",
data_format : "None",
seed : "None",
dtype : "None",
        },
    },
});
    
/// SpatialDropout3D
/// generate by convert_tfk_layer.py
export const SpatialDropout3D =  defineLayerNode({
    node_name: "SpatialDropout3D",
    params: {

        },
    pytorch: {
        class_name: "SpatialDropout3D",
        module: "keras.src.layers.regularization.spatial_dropout",
        params: {
rate : "None",
data_format : "None",
seed : "None",
dtype : "None",
        },
    },
});
    
/// Cropping1D
/// generate by convert_tfk_layer.py
export const Cropping1D =  defineLayerNode({
    node_name: "Cropping1D",
    params: {

        },
    pytorch: {
        class_name: "Cropping1D",
        module: "keras.src.layers.reshaping.cropping1d",
        params: {
cropping : "(1, 1)",
        },
    },
});
    
/// Cropping2D
/// generate by convert_tfk_layer.py
export const Cropping2D =  defineLayerNode({
    node_name: "Cropping2D",
    params: {

        },
    pytorch: {
        class_name: "Cropping2D",
        module: "keras.src.layers.reshaping.cropping2d",
        params: {
cropping : "((0, 0), (0, 0))",
data_format : "None",
        },
    },
});
    
/// Cropping3D
/// generate by convert_tfk_layer.py
export const Cropping3D =  defineLayerNode({
    node_name: "Cropping3D",
    params: {

        },
    pytorch: {
        class_name: "Cropping3D",
        module: "keras.src.layers.reshaping.cropping3d",
        params: {
cropping : "((1, 1), (1, 1), (1, 1))",
data_format : "None",
        },
    },
});
    
/// Flatten
/// generate by convert_tfk_layer.py
export const Flatten =  defineLayerNode({
    node_name: "Flatten",
    params: {

        },
    pytorch: {
        class_name: "Flatten",
        module: "keras.src.layers.reshaping.flatten",
        params: {
data_format : "None",
        },
    },
});
    
/// Permute
/// generate by convert_tfk_layer.py
export const Permute =  defineLayerNode({
    node_name: "Permute",
    params: {

        },
    pytorch: {
        class_name: "Permute",
        module: "keras.src.layers.reshaping.permute",
        params: {
dims : "NoneType",
        },
    },
});
    
/// RepeatVector
/// generate by convert_tfk_layer.py
export const RepeatVector =  defineLayerNode({
    node_name: "RepeatVector",
    params: {

        },
    pytorch: {
        class_name: "RepeatVector",
        module: "keras.src.layers.reshaping.repeat_vector",
        params: {
n : "NoneType",
        },
    },
});
    
/// Reshape
/// generate by convert_tfk_layer.py
export const Reshape =  defineLayerNode({
    node_name: "Reshape",
    params: {

        },
    pytorch: {
        class_name: "Reshape",
        module: "keras.src.layers.reshaping.reshape",
        params: {
target_shape : "NoneType",
        },
    },
});
    
/// UpSampling1D
/// generate by convert_tfk_layer.py
export const UpSampling1D =  defineLayerNode({
    node_name: "UpSampling1D",
    params: {

        },
    pytorch: {
        class_name: "UpSampling1D",
        module: "keras.src.layers.reshaping.up_sampling1d",
        params: {
size : "2",
        },
    },
});
    
/// UpSampling2D
/// generate by convert_tfk_layer.py
export const UpSampling2D =  defineLayerNode({
    node_name: "UpSampling2D",
    params: {

        },
    pytorch: {
        class_name: "UpSampling2D",
        module: "keras.src.layers.reshaping.up_sampling2d",
        params: {
size : "(2, 2)",
data_format : "None",
interpolation : "nearest",
        },
    },
});
    
/// UpSampling3D
/// generate by convert_tfk_layer.py
export const UpSampling3D =  defineLayerNode({
    node_name: "UpSampling3D",
    params: {

        },
    pytorch: {
        class_name: "UpSampling3D",
        module: "keras.src.layers.reshaping.up_sampling3d",
        params: {
size : "(2, 2, 2)",
data_format : "None",
        },
    },
});
    
/// ZeroPadding1D
/// generate by convert_tfk_layer.py
export const ZeroPadding1D =  defineLayerNode({
    node_name: "ZeroPadding1D",
    params: {

        },
    pytorch: {
        class_name: "ZeroPadding1D",
        module: "keras.src.layers.reshaping.zero_padding1d",
        params: {
padding : "1",
        },
    },
});
    
/// ZeroPadding2D
/// generate by convert_tfk_layer.py
export const ZeroPadding2D =  defineLayerNode({
    node_name: "ZeroPadding2D",
    params: {

        },
    pytorch: {
        class_name: "ZeroPadding2D",
        module: "keras.src.layers.reshaping.zero_padding2d",
        params: {
padding : "(1, 1)",
data_format : "None",
        },
    },
});
    
/// ZeroPadding3D
/// generate by convert_tfk_layer.py
export const ZeroPadding3D =  defineLayerNode({
    node_name: "ZeroPadding3D",
    params: {

        },
    pytorch: {
        class_name: "ZeroPadding3D",
        module: "keras.src.layers.reshaping.zero_padding3d",
        params: {
padding : "((1, 1), (1, 1), (1, 1))",
data_format : "None",
        },
    },
});
    
/// Bidirectional
/// generate by convert_tfk_layer.py
export const Bidirectional =  defineLayerNode({
    node_name: "Bidirectional",
    params: {

        },
    pytorch: {
        class_name: "Bidirectional",
        module: "keras.src.layers.rnn.bidirectional",
        params: {
layer : "concat",
merge_mode : "None",
weights : "None",
backward_layer : "NoneType",
        },
    },
});
    
/// ConvLSTM1D
/// generate by convert_tfk_layer.py
export const ConvLSTM1D =  defineLayerNode({
    node_name: "ConvLSTM1D",
    params: {

        },
    pytorch: {
        class_name: "ConvLSTM1D",
        module: "keras.src.layers.rnn.conv_lstm1d",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "tanh",
dilation_rate : "sigmoid",
activation : "True",
recurrent_activation : "glorot_uniform",
use_bias : "orthogonal",
kernel_initializer : "zeros",
recurrent_initializer : "True",
bias_initializer : "None",
unit_forget_bias : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "0.0",
bias_constraint : "0.0",
dropout : "None",
recurrent_dropout : "False",
seed : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "NoneType",
stateful : "NoneType",
        },
    },
});
    
/// ConvLSTM2D
/// generate by convert_tfk_layer.py
export const ConvLSTM2D =  defineLayerNode({
    node_name: "ConvLSTM2D",
    params: {

        },
    pytorch: {
        class_name: "ConvLSTM2D",
        module: "keras.src.layers.rnn.conv_lstm2d",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "tanh",
dilation_rate : "sigmoid",
activation : "True",
recurrent_activation : "glorot_uniform",
use_bias : "orthogonal",
kernel_initializer : "zeros",
recurrent_initializer : "True",
bias_initializer : "None",
unit_forget_bias : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "0.0",
bias_constraint : "0.0",
dropout : "None",
recurrent_dropout : "False",
seed : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "NoneType",
stateful : "NoneType",
        },
    },
});
    
/// ConvLSTM3D
/// generate by convert_tfk_layer.py
export const ConvLSTM3D =  defineLayerNode({
    node_name: "ConvLSTM3D",
    params: {

        },
    pytorch: {
        class_name: "ConvLSTM3D",
        module: "keras.src.layers.rnn.conv_lstm3d",
        params: {
filters : "1",
kernel_size : "valid",
strides : "None",
padding : "1",
data_format : "tanh",
dilation_rate : "sigmoid",
activation : "True",
recurrent_activation : "glorot_uniform",
use_bias : "orthogonal",
kernel_initializer : "zeros",
recurrent_initializer : "True",
bias_initializer : "None",
unit_forget_bias : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "0.0",
bias_constraint : "0.0",
dropout : "None",
recurrent_dropout : "False",
seed : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "NoneType",
stateful : "NoneType",
        },
    },
});
    
/// GRU
/// generate by convert_tfk_layer.py
export const GRU =  defineLayerNode({
    node_name: "GRU",
    params: {

        },
    pytorch: {
        class_name: "GRU",
        module: "keras.src.layers.rnn.gru",
        params: {
units : "tanh",
activation : "sigmoid",
recurrent_activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "None",
seed : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "False",
stateful : "False",
unroll : "True",
reset_after : "auto",
use_cudnn : "NoneType",
        },
    },
});
    
/// GRUCell
/// generate by convert_tfk_layer.py
export const GRUCell =  defineLayerNode({
    node_name: "GRUCell",
    params: {

        },
    pytorch: {
        class_name: "GRUCell",
        module: "keras.src.layers.rnn.gru",
        params: {
units : "tanh",
activation : "sigmoid",
recurrent_activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "True",
reset_after : "None",
seed : "NoneType",
        },
    },
});
    
/// LSTM
/// generate by convert_tfk_layer.py
export const LSTM =  defineLayerNode({
    node_name: "LSTM",
    params: {

        },
    pytorch: {
        class_name: "LSTM",
        module: "keras.src.layers.rnn.lstm",
        params: {
units : "tanh",
activation : "sigmoid",
recurrent_activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "True",
unit_forget_bias : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "None",
seed : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "False",
stateful : "False",
unroll : "auto",
use_cudnn : "NoneType",
        },
    },
});
    
/// LSTMCell
/// generate by convert_tfk_layer.py
export const LSTMCell =  defineLayerNode({
    node_name: "LSTMCell",
    params: {

        },
    pytorch: {
        class_name: "LSTMCell",
        module: "keras.src.layers.rnn.lstm",
        params: {
units : "tanh",
activation : "sigmoid",
recurrent_activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "True",
unit_forget_bias : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "None",
seed : "NoneType",
        },
    },
});
    
/// RNN
/// generate by convert_tfk_layer.py
export const RNN =  defineLayerNode({
    node_name: "RNN",
    params: {

        },
    pytorch: {
        class_name: "RNN",
        module: "keras.src.layers.rnn.rnn",
        params: {
cell : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "False",
stateful : "False",
unroll : "False",
zero_output_for_mask : "NoneType",
        },
    },
});
    
/// SimpleRNN
/// generate by convert_tfk_layer.py
export const SimpleRNN =  defineLayerNode({
    node_name: "SimpleRNN",
    params: {

        },
    pytorch: {
        class_name: "SimpleRNN",
        module: "keras.src.layers.rnn.simple_rnn",
        params: {
units : "tanh",
activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
activity_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "False",
return_sequences : "False",
return_state : "False",
go_backwards : "False",
stateful : "False",
unroll : "None",
seed : "NoneType",
        },
    },
});
    
/// SimpleRNNCell
/// generate by convert_tfk_layer.py
export const SimpleRNNCell =  defineLayerNode({
    node_name: "SimpleRNNCell",
    params: {

        },
    pytorch: {
        class_name: "SimpleRNNCell",
        module: "keras.src.layers.rnn.simple_rnn",
        params: {
units : "tanh",
activation : "True",
use_bias : "glorot_uniform",
kernel_initializer : "orthogonal",
recurrent_initializer : "zeros",
bias_initializer : "None",
kernel_regularizer : "None",
recurrent_regularizer : "None",
bias_regularizer : "None",
kernel_constraint : "None",
recurrent_constraint : "None",
bias_constraint : "0.0",
dropout : "0.0",
recurrent_dropout : "None",
seed : "NoneType",
        },
    },
});
    
/// StackedRNNCells
/// generate by convert_tfk_layer.py
export const StackedRNNCells =  defineLayerNode({
    node_name: "StackedRNNCells",
    params: {

        },
    pytorch: {
        class_name: "StackedRNNCells",
        module: "keras.src.layers.rnn.stacked_rnn_cells",
        params: {
cells : "NoneType",
        },
    },
});
    
/// TimeDistributed
/// generate by convert_tfk_layer.py
export const TimeDistributed =  defineLayerNode({
    node_name: "TimeDistributed",
    params: {

        },
    pytorch: {
        class_name: "TimeDistributed",
        module: "keras.src.layers.rnn.time_distributed",
        params: {
layer : "NoneType",
        },
    },
});
    
/// AlphaDropout
/// generate by convert_tfk_layer.py
export const AlphaDropout =  defineLayerNode({
    node_name: "AlphaDropout",
    params: {

        },
    pytorch: {
        class_name: "AlphaDropout",
        module: "keras.src.legacy.layers",
        params: {
rate : "None",
noise_shape : "None",
seed : "NoneType",
        },
    },
});
    
/// RandomHeight
/// generate by convert_tfk_layer.py
export const RandomHeight =  defineLayerNode({
    node_name: "RandomHeight",
    params: {

        },
    pytorch: {
        class_name: "RandomHeight",
        module: "keras.src.legacy.layers",
        params: {
factor : "bilinear",
interpolation : "None",
seed : "NoneType",
        },
    },
});
    
/// RandomWidth
/// generate by convert_tfk_layer.py
export const RandomWidth =  defineLayerNode({
    node_name: "RandomWidth",
    params: {

        },
    pytorch: {
        class_name: "RandomWidth",
        module: "keras.src.legacy.layers",
        params: {
factor : "bilinear",
interpolation : "None",
seed : "NoneType",
        },
    },
});
    
/// ThresholdedReLU
/// generate by convert_tfk_layer.py
export const ThresholdedReLU =  defineLayerNode({
    node_name: "ThresholdedReLU",
    params: {

        },
    pytorch: {
        class_name: "ThresholdedReLU",
        module: "keras.src.legacy.layers",
        params: {
theta : "1.0",
        },
    },
});
    
/// FlaxLayer
/// generate by convert_tfk_layer.py
export const FlaxLayer =  defineLayerNode({
    node_name: "FlaxLayer",
    params: {

        },
    pytorch: {
        class_name: "FlaxLayer",
        module: "keras.src.utils.jax_layer",
        params: {
module : "None",
method : "None",
variables : "NoneType",
        },
    },
});
    
/// JaxLayer
/// generate by convert_tfk_layer.py
export const JaxLayer =  defineLayerNode({
    node_name: "JaxLayer",
    params: {

        },
    pytorch: {
        class_name: "JaxLayer",
        module: "keras.src.utils.jax_layer",
        params: {
call_fn : "None",
init_fn : "None",
params : "None",
state : "None",
seed : "NoneType",
        },
    },
});
    
/// TorchModuleWrapper
/// generate by convert_tfk_layer.py
export const TorchModuleWrapper =  defineLayerNode({
    node_name: "TorchModuleWrapper",
    params: {

        },
    pytorch: {
        class_name: "TorchModuleWrapper",
        module: "keras.src.utils.torch_utils",
        params: {
module : "None",
        },
    },
});
    