<template>
  <TDTable :columns="currentColumns" :data="currentData" rowKey="key" bordered>
    <template #registry="{ row }">
      <TDLink target="_blank" :href="row.registry" :theme="refreshIconType" :underline="!row?.disabled"
        :disabled="row?.disabled">
        {{ row.registry }}
      </TDLink>
    </template>
    <template #action>
      <RefreshIcon class="action" @click="getRegistry" :style="{ color: getTheme(refreshIconType) }" />
    </template>
  </TDTable>
  <TDDivider>
    <DogeIcon />
  </TDDivider>
  <TDTable :columns="registryColumns" :data="registryData" rowKey="key" bordered>
    <template #registry="{ row }">
      <TDLink target="_blank" :href="row.registry" theme="success" underline>{{ row.registry }}</TDLink>
    </template>
    <template #action="{ row }">
      <InstallDesktopIcon class="action" @click="installNpmRegistry(row)"
        :style="{ color: installDesktopIconType.key === row.key ? getTheme(installDesktopIconType.type) : '' }" />
    </template>
  </TDTable>
</template>

<script setup lang="ts">
import { Table as TDTable, Link as TDLink, Divider as TDDivider, TableProps } from "tdesign-vue-next";
import { RefreshIcon, DogeIcon, InstallDesktopIcon } from 'tdesign-icons-vue-next';
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from "vue";

type TableData = {
  key: number;
  registry: string;
  disabled?: boolean;
};

enum RefreshIconType {
  SUCCESS = 'success',
  FAILED = 'danger',
  LOADING = 'warning',
  WAITING = 'default',
}

const refreshIconType = ref<RefreshIconType>(RefreshIconType.WAITING);
const installDesktopIconType = ref<{ key: number; type: RefreshIconType }>({ key: 0, type: RefreshIconType.WAITING });

const currentColumns: TableProps['columns'] = [
  { colKey: 'registry', title: '当前源', align: 'center' },
  { colKey: 'action', title: '刷新', width: 200, align: 'center' },
];

const currentData = ref<TableData[]>([{ key: 0, registry: 'Check the npm version', disabled: true }]);

const registryColumns: TableProps['columns'] = [
  { colKey: 'registry', title: '可用源', align: 'center' },
  { colKey: 'action', title: '安装', width: 200, align: 'center' },
];

const registryData = [
  { key: 0, registry: 'https://registry.npmjs.org' },
  { key: 1, registry: 'https://registry.npmmirror.com' },
  { key: 2, registry: 'http://mirrors.cloud.tencent.com/npm' },
  { key: 3, registry: 'https://repo.huaweicloud.com/repository/npm/' },
];

const getTheme = (type: RefreshIconType) => {
  switch (type) {
    case RefreshIconType.SUCCESS: return 'green';
    case RefreshIconType.FAILED: return 'red';
    case RefreshIconType.LOADING: return '#e37318';
    default: return 'black';
  }
};

/**
 * 
 * @param cmdCommand 
 * @returns 
 * @description execute cmd command
 */
const executeCmdCommand = async (cmdCommand: string): Promise<string> => await invoke('execute_cmd_command', { cmdCommand });

/**
 * 
 * @description get npm registry
 */
const getRegistry = async () => {
  refreshIconType.value = RefreshIconType.LOADING;
  const registry = await executeCmdCommand('npm config get registry');
  if (registry === 'failed') {
    currentData.value = [{ key: 0, registry: 'npm source retrieval failed', disabled: true }];
    refreshIconType.value = RefreshIconType.FAILED;
  } else {
    currentData.value = [{ key: 0, registry }];
    refreshIconType.value = RefreshIconType.SUCCESS;
  }
};

/**
 * 
 * @param row 
 * @description npm registry install
 */
const installNpmRegistry = async (row: TableData) => {
  installDesktopIconType.value = { key: row.key, type: RefreshIconType.LOADING };
  const installNpmRegistryType = await executeCmdCommand(`npm config set registry ${row.registry}`);

  if (installNpmRegistryType !== 'failed') {
    const getRegistry = await executeCmdCommand('npm config get registry');

    if (getRegistry === 'failed') {
      currentData.value = [{ key: 0, registry: 'npm source retrieval failed' }];
      refreshIconType.value = RefreshIconType.FAILED;
    } else {
      currentData.value = [{ key: 0, registry: getRegistry }];
      installDesktopIconType.value = { key: row.key, type: RefreshIconType.SUCCESS };
    }
  } else {
    installDesktopIconType.value = { key: row.key, type: RefreshIconType.FAILED };
  }
};

onMounted(() => getRegistry());
</script>

<style scoped>
.action {
  cursor: pointer;
}
</style>