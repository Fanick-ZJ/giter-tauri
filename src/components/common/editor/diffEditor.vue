<script setup lang="ts">
import { loadMonaco } from "@/composables/useMonaco";
import { useThemeStore } from "@/store/modules/theme";
import { getMonacoLanguage } from "@/utils/tool";
import type * as monaco from "monaco-editor";
import { getCurrentInstance, onMounted, ref, watch } from "vue";
import { FileOption } from "./types";

const props = defineProps<{
  original: FileOption;
  modified: FileOption;
  readonly: boolean;
}>();

const instance = getCurrentInstance();
let editor: monaco.editor.IStandaloneDiffEditor;
let monacoApi: typeof monaco;
const editorBody = ref<HTMLElement>();
const themeStore = useThemeStore();

const autoDetectLanguage = (type: "original" | "modified") => {
  const hasLanguage =
    instance?.vnode.props?.[type]["language"] != undefined;
  if (hasLanguage) {
    return instance?.vnode.props?.[type]["language"];
  }
  return getMonacoLanguage(
    instance?.vnode.props?.[type]["filename"] || "",
  );
};

const useEditorModel = () => {
  const originModel = monacoApi.editor.createModel(
    props.original.content,
    autoDetectLanguage("original"),
  );
  const modifiedModel = monacoApi.editor.createModel(
    props.modified.content,
    autoDetectLanguage("modified"),
  );

  watch(
    () => props.original.content,
    (newVal) => {
      originModel.setValue(newVal!);
    },
  );

  watch(
    () => props.modified.content,
    (newVal) => {
      modifiedModel.setValue(newVal!);
    },
  );

  return { originModel, modifiedModel };
};

// 注意：useEditorModel 依赖 monacoApi，需在 initEditor 中调用
let originModel: monaco.editor.ITextModel;
let modifiedModel: monaco.editor.ITextModel;

// 监听主题变化
watch(
  () => themeStore.isDark,
  (isDark) => {
    if (editor && monacoApi) {
      monacoApi.editor.setTheme(isDark ? "vs-dark" : "vs");
    }
  },
);

const initEditor = () => {
  if (!editorBody.value || !monacoApi) return;

  originModel = monacoApi.editor.createModel(
    props.original.content,
    autoDetectLanguage("original"),
  );
  modifiedModel = monacoApi.editor.createModel(
    props.modified.content,
    autoDetectLanguage("modified"),
  );

  // 监听 content 变化
  watch(
    () => props.original.content,
    (newVal) => {
      originModel.setValue(newVal!);
    },
  );
  watch(
    () => props.modified.content,
    (newVal) => {
      modifiedModel.setValue(newVal!);
    },
  );

  editor = monacoApi.editor.createDiffEditor(editorBody.value, {
    theme: themeStore.isDark ? "vs-dark" : "vs",
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    automaticLayout: true,
    scrollbar: {
      horizontal: "auto",
      handleMouseWheel: true,
      alwaysConsumeMouseWheel: false,
    },
    readOnly: props.readonly,
    contextmenu: false,
  });

  editor.setModel({
    original: originModel,
    modified: modifiedModel,
  });
};

onMounted(async () => {
  monacoApi = await loadMonaco();
  initEditor();
});
</script>

<template>
  <div class="editor-container w-full h-full flex flex-col">
    <div class="editor-header">
      <slot name="header" />
    </div>
    <div class="flex-1 relative">
      <div
        class="editor-body absolute w-full h-full flex-auto"
        ref="editorBody"
      ></div>
    </div>
  </div>
</template>
