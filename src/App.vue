<template>
  <div
    tabindex="0"
    @keydown="onKeyDown"
    class="inputArea"
    :class="{ unAllow: unAllow }"
  >
    <div v-if="bindKey">
      {{ bindKey }}
    </div>
    <div class="hint" v-else>Press Any Key Here</div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import { readTextFile, writeFile } from "@tauri-apps/api/fs";
import { dataDir, resolve } from "@tauri-apps/api/path";
import {keymap} from "./keymap";

import {
  register as registerShortcut,
  unregister as unregisterShortcut,
  unregisterAll as unregisterAllShortcuts,
} from "@tauri-apps/api/globalShortcut";

const bindKey = ref("");
const unAllow = ref(false);

function onKeyDown(ev:any) {
  bindKey.value = ev.code;
}

watch(bindKey, (val) => {
  let shortcut = keymap[val];
  unAllow.value = !shortcut;

  unregisterAllShortcuts().then(() => {});
  if (val.trim().length > 0 && shortcut) {
    registerShortcut(shortcut, () => {
      invoke("trigger_click");

      console.log(`Shortcut ${shortcut} triggered`);
    })
      .then(() => {
        writeLocalData(val);
        console.log(`Shortcut ${shortcut} registered successfully`);
      })
      .catch(() => {
        console.log(`${shortcut} registered wrong`);
      });
  }
});

function writeLocalData(shortcut: string) {
  return new Promise((res, rej) => {
    dataDir()
      .then((dataFolder) => {
        writeFile({
          contents: shortcut,
          path: dataFolder + "SClick.txt",
        })
          .then((s) => {
            console.log("write localdata done");
          })
          .catch(() => {
            console.log("write faild");
          });
      })
      .catch((e) => rej());
  });
}

function getLocalData() {
  return new Promise((res, rej) => {
    dataDir()
      .then((dataFolder) => {
        resolve(dataFolder, "./SClick.txt")
          .then((filePath) => {
            console.log(filePath, "config path");

            readTextFile(filePath)
              .then((content) => {
                res(content);
              })
              .catch((e) => rej());
          })
          .catch((e) => rej());
      })
      .catch((e) => rej());
  });
}

onMounted(() => {
  getLocalData()
    .then((content) => {
      bindKey.value = content as string;
    })
    .catch((e) => {
      console.log("读取配置失败，等待用户自行配置");
    });
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

.inputArea {
  width: 200px;
  height: 60px;
  text-align: center;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
    Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  font-size: 24px;
  line-height: 60px;
  color: #444;
  outline: none;
}

.inputArea:focus {
  background: rgb(225, 230, 255);
}

.hint {
  font-size: 18px;
}

.inputArea.unAllow{
  background: #f20;
  color: #fff;
}
</style>
