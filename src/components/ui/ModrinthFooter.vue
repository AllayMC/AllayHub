<script setup lang="ts">
import {
  defineMessage,
  defineMessages,
  IntlFormatted,
  type MessageDescriptor,
  useVIntl,
} from '@modrinth/ui'
const { formatMessage } = useVIntl()

const messages = defineMessages({
  projectAttribution: {
    id: 'layout.footer.project-attribution',
    defaultMessage:
      "This project is a derivative work based on <allayhub-link>AllayMC's open-source project AllayHub</allayhub-link> and is open-sourced under the AGPL license at <nukkithub-link>MemoriesOfTime/NukkitHub</nukkithub-link>.",
  },
  legalDisclaimer: {
    id: 'layout.footer.legal-disclaimer',
    defaultMessage:
      'NOT AN OFFICIAL MINECRAFT SERVICE. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.',
  },
})

const footerLinks: {
  label: MessageDescriptor
  links: {
    href: string
    label: MessageDescriptor
  }[]
}[] = [
  {
    label: defineMessage({
      id: 'layout.footer.allay',
      defaultMessage: 'Allay',
    }),
    links: [
      {
        href: 'https://github.com/MemoriesOfTime/Nukkit-MOT',
        label: defineMessage({
          id: 'layout.footer.allay.server',
          defaultMessage: 'Nukkit-MOT Server',
        }),
      },
      {
        href: 'https://www.nukkit-mot.com/docs/intro',
        label: defineMessage({
          id: 'layout.footer.allay.docs',
          defaultMessage: 'Documentation',
        }),
      },
    ],
  },
  {
    label: defineMessage({
      id: 'layout.footer.products',
      defaultMessage: 'Products',
    }),
    links: [
      {
        href: 'https://github.com/MemoriesOfTime/ExamplePlugin-Maven',
        label: defineMessage({
          id: 'layout.footer.products.plugins',
          defaultMessage: 'Plugins',
        }),
      },
    ],
  },
  {
    label: defineMessage({
      id: 'layout.footer.resources',
      defaultMessage: 'Resources',
    }),
    links: [
      {
        href: 'https://github.com/MemoriesOfTime/NukkitHub/issues',
        label: defineMessage({
          id: 'layout.footer.resources.report-issues',
          defaultMessage: 'Report issues',
        }),
      },
      {
        href: 'https://github.com/MemoriesOfTime/NukkitHub',
        label: defineMessage({
          id: 'layout.footer.resources.source-code',
          defaultMessage: 'Source Code',
        }),
      },
    ],
  },
  {
    label: defineMessage({
      id: 'layout.footer.community',
      defaultMessage: 'Community',
    }),
    links: [
      {
        href: 'https://discord.gg/pJjQDQC',
        label: defineMessage({
          id: 'layout.footer.community.discord',
          defaultMessage: 'Discord',
        }),
      },
      {
        href: 'https://github.com/MemoriesOfTime',
        label: defineMessage({
          id: 'layout.footer.community.github',
          defaultMessage: 'GitHub',
        }),
      },
    ],
  },
]

</script>

<template>
  <footer
    class="footer-brand-background experimental-styles-within border-0 border-t-[1px] border-solid"
  >
    <div
      class="mx-auto flex max-w-screen-xl flex-col gap-6 p-6 pb-20 sm:px-12 md:py-12"
    >
      <div class="grid grid-cols-1 gap-6 text-primary sm:grid-cols-2 lg:grid-cols-4">
        <div
          v-for="group in footerLinks"
          :key="group.label.id"
          class="flex flex-col items-center gap-3 sm:items-start"
        >
          <h3 class="m-0 text-base text-contrast">
            {{ formatMessage(group.label) }}
          </h3>
          <template v-for="item in group.links" :key="item.label">
            <nuxt-link
              v-if="item.href.startsWith('/')"
              :to="item.href"
              class="w-fit hover:underline"
            >
              {{ formatMessage(item.label) }}
            </nuxt-link>
            <a
              v-else
              :href="item.href"
              class="w-fit hover:underline"
              target="_blank"
              rel="noopener"
            >
              {{ formatMessage(item.label) }}
            </a>
          </template>
        </div>
      </div>
      <div
        class="flex justify-center text-center text-xs font-medium text-secondary opacity-50"
      >
        <p class="m-0">
          <IntlFormatted :message-id="messages.projectAttribution">
            <template #allayhub-link="{ children }">
              <a
                href="https://github.com/AllayMC/AllayHub"
                class="text-brand hover:underline"
                target="_blank"
                rel="noopener"
              >
                <component :is="() => children" />
              </a>
            </template>
            <template #nukkithub-link="{ children }">
              <a
                href="https://github.com/MemoriesOfTime/NukkitHub"
                class="text-brand hover:underline"
                target="_blank"
                rel="noopener"
              >
                <component :is="() => children" />
              </a>
            </template>
          </IntlFormatted>
        </p>
      </div>
      <div
        class="flex justify-center text-center text-xs font-medium text-secondary opacity-50"
      >
        {{ formatMessage(messages.legalDisclaimer) }}
      </div>
    </div>
  </footer>
</template>
<style scoped lang="scss">
.footer-brand-background {
  background: var(--brand-gradient-strong-bg);
  border-color: var(--brand-gradient-border);
}
</style>
