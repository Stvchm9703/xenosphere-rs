<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import NodeGraph from "./components/node_graph/main.vue";
</script>

<template>
  <div class="container">
    <NodeGraph></NodeGraph> 
    </div>
</template>

<style>
body {
  margin: 0;
}

</style>

<style scoped>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  display: block;
 
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
}


</style>
