<script setup lang="ts" generic="T">
import { DropdownIcon } from '@modrinth/assets'
import type { Ref } from 'vue'
import { computed, ref } from 'vue'

import Checkbox from '../base/Checkbox.vue'
import type { ContentItem } from './ContentListItem.vue'
import ContentListItem from './ContentListItem.vue'

const props = withDefaults(
  defineProps<{
    items: ContentItem<T>[]
    sortColumn: string
    sortAscending: boolean
    updateSort: (column: string) => void
    currentPage: number
  }>(),
  {},
)

const selectionStates: Ref<Record<string, boolean>> = ref({})
const selected: Ref<string[]> = computed(() =>
  Object.keys(selectionStates.value).filter(
    (item) =>
      selectionStates.value[item] &&
      props.items.some((x) => x.filename === item),
  ),
)

const allSelected = ref(false)

const model = defineModel<string[]>()

function updateSelection() {
  model.value = selected.value
}

function setSelected(value: boolean) {
  if (value) {
    selectionStates.value = Object.fromEntries(
      props.items.map((item) => [item.filename, true]),
    )
  } else {
    selectionStates.value = {}
  }
  updateSelection()
}

const paginatedItems = computed(() =>
  props.items.slice((props.currentPage - 1) * 20, props.currentPage * 20),
)
</script>

<template>
  <div class="flex grid-cols-[min-content,auto,auto,auto,auto] flex-col">
    <div
      :class="`${$slots.headers ? 'flex' : 'grid'} mb-3 h-10 grid-cols-[min-content,4fr,3fr,2fr] items-center gap-3 px-2 pt-1 font-bold text-contrast`"
    >
      <Checkbox
        v-model="allSelected"
        class="select-checkbox"
        :indeterminate="selected.length > 0 && selected.length < items.length"
        @update:model-value="setSelected"
      />
      <slot name="headers">
        <div
          class="flex cursor-pointer items-center gap-2"
          @click="updateSort('Name')"
        >
          Name
          <DropdownIcon
            v-if="sortColumn === 'Name'"
            class="transform transition-all"
            :class="{ 'rotate-180': sortAscending }"
          />
        </div>
        <div
          class="flex max-w-60 cursor-pointer items-center gap-1"
          @click="updateSort('Updated')"
        >
          Updated
          <DropdownIcon
            v-if="sortColumn === 'Updated'"
            class="transform transition-all"
            :class="{ 'rotate-180': sortAscending }"
          />
        </div>
        <div class="flex justify-end gap-2">
          <slot name="header-actions" />
        </div>
      </slot>
    </div>
    <div class="rounded-xl bg-bg-raised">
      <ContentListItem
        v-for="(itemRef, index) in paginatedItems"
        :key="itemRef.filename"
        v-model="selectionStates[itemRef.filename]"
        :item="itemRef"
        :last="index === paginatedItems.length - 1"
        class="mb-2"
        @update:model-value="updateSelection"
      >
        <template #actions="{ item }">
          <slot name="actions" :item="item" />
        </template>
      </ContentListItem>
    </div>
  </div>
</template>
