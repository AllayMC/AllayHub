<script setup lang="ts">
import { type Component, nextTick, ref } from 'vue'

import { type MessageDescriptor, useVIntl } from '../../composables/i18n'
import { useScrollIndicator } from '../../composables/scroll-indicator'

const { formatMessage } = useVIntl()

export type Tab<Props> = {
  name: MessageDescriptor
  icon: Component
  content: Component<Props>
  props?: Props
  badge?: MessageDescriptor
}

defineProps<{
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  tabs: Tab<any>[]
}>()

const selectedTab = ref(0)

const scrollContainer = ref<HTMLElement | null>(null)
const { showTopFade, showBottomFade, checkScrollState, forceCheck } =
  useScrollIndicator(scrollContainer)

function setTab(index: number) {
  selectedTab.value = index
  nextTick(() => forceCheck())
}

defineExpose({ selectedTab, setTab })
</script>
<template>
  <div class="grid grid-cols-[auto_1fr]">
    <div
      class="flex min-w-[200px] flex-col gap-1 border-0 border-r-[1px] border-solid border-divider pr-4"
    >
      <button
        v-for="(tab, index) in tabs"
        :key="index"
        :class="`flex cursor-pointer items-center gap-2 text-nowrap rounded-xl border-none px-4 py-2 text-left font-semibold transition-all active:scale-[0.97] ${selectedTab === index ? 'bg-button-bgSelected text-button-textSelected' : 'bg-transparent text-button-text hover:bg-button-bg hover:text-contrast'}`"
        @click="() => setTab(index)"
      >
        <component :is="tab.icon" class="h-4 w-4" />
        <span>{{ formatMessage(tab.name) }}</span>
        <span
          v-if="tab.badge"
          class="rounded-full bg-brand-highlight px-1.5 py-0.5 text-xs font-bold text-brand-green"
        >
          {{ formatMessage(tab.badge) }}
        </span>
      </button>

      <slot name="footer" />
    </div>
    <div class="relative">
      <Transition
        enter-active-class="transition-all duration-200 ease-out"
        enter-from-class="opacity-0 max-h-0"
        enter-to-class="opacity-100 max-h-24"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 max-h-24"
        leave-to-class="opacity-0 max-h-0"
      >
        <div
          v-if="showTopFade"
          class="pointer-events-none absolute left-0 right-0 top-0 z-10 h-24 bg-gradient-to-b from-bg-raised to-transparent"
        />
      </Transition>

      <div
        ref="scrollContainer"
        class="h-[500px] w-[600px] overflow-y-auto px-4"
        @scroll="checkScrollState"
      >
        <component
          :is="tabs[selectedTab].content"
          v-bind="tabs[selectedTab].props ?? {}"
        />
      </div>

      <Transition
        enter-active-class="transition-all duration-200 ease-out"
        enter-from-class="opacity-0 max-h-0"
        enter-to-class="opacity-100 max-h-24"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 max-h-24"
        leave-to-class="opacity-0 max-h-0"
      >
        <div
          v-if="showBottomFade"
          class="pointer-events-none absolute bottom-0 left-0 right-0 z-10 h-24 bg-gradient-to-t from-bg-raised to-transparent"
        />
      </Transition>
    </div>
  </div>
</template>
