import { InjectionKey, Ref } from "vue";

export const viewExtend = Symbol() as InjectionKey<() => void>
export const viewShrink = Symbol() as InjectionKey<() => void>