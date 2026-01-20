<template>
  <div
    :class="[
      'flex flex-col gap-3 rounded-2xl border-[1px] border-solid p-4 text-contrast',
      typeClasses[type],
    ]"
  >
    <div
      :class="[
        'flex items-start gap-2',
        (header || $slots.header) && 'flex-col',
      ]"
    >
      <div
        class="flex items-start gap-2"
        :class="header || $slots.header ? 'w-full' : 'contents'"
      >
        <slot
          name="icon"
          :icon-class="['h-6 w-6 flex-none', iconClasses[type]]"
        >
          <component
            :is="getSeverityIcon(type)"
            :class="['h-6 w-6 flex-none', iconClasses[type]]"
          />
        </slot>
        <div v-if="header || $slots.header" class="text-base font-semibold">
          <slot name="header">{{ header }}</slot>
        </div>
      </div>
      <div
        class="text-base font-normal"
        :class="!(header || $slots.header) && 'flex-1'"
      >
        <slot>{{ body }}</slot>
      </div>
    </div>
    <div v-if="showActionsUnderneath || $slots.actions">
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { getSeverityIcon } from '../../utils'

withDefaults(
  defineProps<{
    type?: 'info' | 'warning' | 'critical'
    header?: string
    body?: string
    showActionsUnderneath?: boolean
  }>(),
  {
    type: 'info',
    header: '',
    body: '',
    showActionsUnderneath: false,
  },
)

const typeClasses = {
  info: 'border-brand-blue bg-bg-blue',
  warning: 'border-brand-orange bg-bg-orange',
  critical: 'border-brand-red bg-bg-red',
}

const iconClasses = {
  info: 'text-brand-blue',
  warning: 'text-brand-orange',
  critical: 'text-brand-red',
}
</script>
