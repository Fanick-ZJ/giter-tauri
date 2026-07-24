<script setup lang="ts">
import { loadMonaco } from "@/composables/useMonaco";
import { useThemeStore } from "@/store/modules/theme";
import { getMonacoLanguage } from "@/utils/tool";
import type * as monaco from "monaco-editor";
import { getCurrentInstance, onMounted, ref, watch } from "vue";

const props = defineProps({
  language: {
    type: String,
    default: "plain-text",
  },
  filename: {
    type: String,
    required: false,
  },
  content: {
    type: String,
    default: "",
  },
  readonly: {
    type: Boolean,
    default: false,
  },
});

const instance = getCurrentInstance();
let editor: monaco.editor.IStandaloneCodeEditor;
let monacoApi: typeof monaco;
const editorBody = ref<HTMLElement>();
const themeStore = useThemeStore();

const autoDetectLanguage = () => {
  const hasLanguage = instance?.vnode.props?.["language"] != undefined;
  if (hasLanguage) {
    return instance?.vnode.props?.["language"];
  }
  return getMonacoLanguage(props.filename || "");
};

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

  editor = monacoApi.editor.create(editorBody.value, {
    value: props.content,
    language: autoDetectLanguage(),
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
  editor.getDomNode()!.style.width = "100%";
  editor.getDomNode()!.style.height = "100%";
  updateEditorHeight();
};

const updateEditorHeight = () => {
  if (!editor || !monacoApi) return;
  const height = calculateEditorHeight();
  editor.getDomNode()!.style.height = `${height}px`;
  editor.layout();
};

function calculateEditorHeight() {
  const lineHeight = editor.getOption(
    monacoApi.editor.EditorOption.lineHeight,
  );
  const lineCount = editor.getModel()!.getLineCount();
  const scrollBeyondLastLine = editor.getOption(
    monacoApi.editor.EditorOption.scrollBeyondLastLine,
  );

  let totalHeight = lineHeight * lineCount;
  if (scrollBeyondLastLine) {
    totalHeight += lineHeight;
  }

  const containerHeight =
    editorBody.value?.parentElement?.clientHeight || 0;
  return Math.min(totalHeight, containerHeight) - 50;
}

onMounted(async () => {
  monacoApi = await loadMonaco();
  initEditor();
});

watch(
  () => props.content,
  (newValue) => {
    if (!editor || !monacoApi) return;
    editor.setValue(newValue);
    monacoApi.editor.setModelLanguage(editor.getModel()!, autoDetectLanguage());
  },
);
</script>

<template>
  <div class="editor-container w-full h-full flex flex-col">
    <div class="editor-header">
      <slot name="header" />
    </div>
    <div class="flex-1 min-h-0 relative">
      <div
        class="editor-body absolute w-full h-full flex-auto"
        ref="editorBody"
      ></div>
    </div>
  </div>
</template>
