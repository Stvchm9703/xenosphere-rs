<template>
  <BaklavaEditor :view-model="baklava" />
</template>
<script setup lang="ts">
import { BaklavaEditor, useBaklava } from "@baklavajs/renderer-vue";
import "@baklavajs/themes/dist/syrup-dark.css";
import { initNodeModules, initLazyNodeModules } from "../../node_graph/nodes/layer/mod";
import { ValueNode } from "../../node_graph/nodes/model_util/valuable";

const baklava = useBaklava();
// initNodeModules(baklava);
initLazyNodeModules(baklava);
baklava.editor.registerNodeType(ValueNode, { category: "Model Creation", });
// baklava.editor.registerNodeType(layerNodes.MaxPool2d, { category: "Layers", });
</script>

