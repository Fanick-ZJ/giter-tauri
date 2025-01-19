import { defineStore } from "pinia";
import { SetupStoreId } from "@/enum";
import { computed, effectScope, onScopeDispose, ref, watch } from "vue";
import { usePreferredColorScheme } from "@vueuse/core";
import { set_config_value } from "@/utils/config";

type Theme = "system" | "dark" | "light";

export const useThemeStore = defineStore(SetupStoreId.Theme, () => {
  const scope = effectScope();
  const osTheme = usePreferredColorScheme();

  /** Theme */
  const theme = ref<string>("system");
  /** Theme */
  const isDark = computed(() => {
    if (theme.value === "dark") {
      return true;
    } else if (theme.value === "light") {
      return false;
    } else {
      return osTheme.value === "dark";
    }
  });

  scope.run(() => {
    watch(
      () => osTheme.value,
      (value) => {
        // 如果是系统主题，则跟随系统
        if (theme.value === "system") {
          theme.value = value;
        }
      }
    );
  });
  /** On scope dispose */
  onScopeDispose(() => {
    scope.stop();
  });
  /** Set theme */
  const setTheme = (value: Theme) => {
    theme.value = value;
    set_config_value("theme", value);
  };

  const toggleTheme = () => {
    if (theme.value === "dark") {
      setTheme("light");
    } else {
      setTheme("dark");
    }
  };

  const toggleDark = () => {
    setTheme("dark");
  };

  return {
    theme,
    isDark,
    setTheme,
    toggleTheme,
    toggleDark
  }
})