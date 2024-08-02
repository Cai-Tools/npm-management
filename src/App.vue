<template>
  <TDTabs :value="selectedTab" @change="selectTab" placement="left" theme="card" lazy>
    <TDTabPanel :value=0 label="npm Registry">
      <NpmRegistry />
    </TDTabPanel>
    <TDTabPanel :value=1 label="node version" disabled lazy>
      node version
    </TDTabPanel>
    <TDTabPanel :value=2 label="package manager" disabled lazy>
      package manager
    </TDTabPanel>
  </TDTabs>
</template>

<script setup lang="ts">
import { Tabs as TDTabs, TabPanel as TDTabPanel, TabValue } from "tdesign-vue-next";
import NpmRegistry from "./page/npm-registry/index.vue";
import { ref } from "vue";

const selectedTab = ref<TabValue>(0);

const selectTab = (value: TabValue) => {
  selectedTab.value = value;
}

</script>

<style scoped>
:deep(.t-tabs__nav-wrap) {
  height: 100vh;
}
</style>