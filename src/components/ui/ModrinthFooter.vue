<script setup lang="ts">
import {
  BlueskyIcon,
  DiscordIcon,
  GithubIcon,
  MastodonIcon,
  TwitterIcon,
} from '@modrinth/assets'
import {
  ButtonStyled,
  defineMessage,
  defineMessages,
  IntlFormatted,
  type MessageDescriptor,
  useVIntl,
} from '@modrinth/ui'

import TextLogo from '~/components/brand/TextLogo.vue'

const flags = useFeatureFlags()
const { formatMessage } = useVIntl()

const messages = defineMessages({
  modrinthInformation: {
    id: 'layout.footer.allayhub-information',
    defaultMessage: 'AllayHub information',
  },
  openSource: {
    id: 'layout.footer.open-source',
    defaultMessage: 'AllayHub is <github-link>open source</github-link>.',
  },
  legalDisclaimer: {
    id: 'layout.footer.legal-disclaimer',
    defaultMessage:
      'NOT AN OFFICIAL MINECRAFT SERVICE. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.',
  },
})

const socialLinks: {
  label: MessageDescriptor
  href: string
  icon: Component
  rel?: string
}[] = [
  {
    label: defineMessage({
      id: 'layout.footer.social.discord',
      defaultMessage: 'Discord',
    }),
    href: 'https://discord.gg/ngkkE4hPTU',
    icon: DiscordIcon,
  },
  {
    label: defineMessage({
      id: 'layout.footer.social.github',
      defaultMessage: 'GitHub',
    }),
    href: 'https://github.com/AllayMC',
    icon: GithubIcon,
  },
  {
    label: defineMessage({
      id: 'layout.footer.social.twitter',
      defaultMessage: 'Twitter',
    }),
    href: '',
    icon: TwitterIcon,
  },
  {
    label: defineMessage({
      id: 'layout.footer.social.mastodon',
      defaultMessage: 'Mastodon',
    }),
    href: '',
    icon: MastodonIcon,
    rel: 'me',
  },
  {
    label: defineMessage({
      id: 'layout.footer.social.bluesky',
      defaultMessage: 'Bluesky',
    }),
    href: '',
    icon: BlueskyIcon,
  },
]

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
        href: 'https://github.com/AllayMC/Allay',
        label: defineMessage({
          id: 'layout.footer.allay.server',
          defaultMessage: 'Allay Server',
        }),
      },
      {
        href: 'https://docs.allaymc.org/',
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
        href: '/app',
        label: defineMessage({
          id: 'layout.footer.products.app',
          defaultMessage: 'AllayLauncher',
        }),
      },
      {
        href: 'https://github.com/AllayMC/AllayGradle',
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
        href: 'https://github.com/AllayMC/AllayHub/issues',
        label: defineMessage({
          id: 'layout.footer.resources.report-issues',
          defaultMessage: 'Report issues',
        }),
      },
      {
        href: 'https://github.com/AllayMC/AllayHub',
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
        href: 'https://discord.gg/ngkkE4hPTU',
        label: defineMessage({
          id: 'layout.footer.community.discord',
          defaultMessage: 'Discord',
        }),
      },
      {
        href: 'https://github.com/AllayMC',
        label: defineMessage({
          id: 'layout.footer.community.github',
          defaultMessage: 'GitHub',
        }),
      },
    ],
  },
]

const developerModeCounter = ref(0)

// Use current year directly instead of GeneratedState
const buildYear = new Date().getFullYear()

function developerModeIncrement() {
  if (developerModeCounter.value >= 5) {
    flags.value.developerMode = !flags.value.developerMode
    developerModeCounter.value = 0
    saveFeatureFlags()
  } else {
    developerModeCounter.value++
  }
}
</script>

<template>
  <footer
    class="footer-brand-background experimental-styles-within border-0 border-t-[1px] border-solid"
  >
    <div
      class="mx-auto flex max-w-screen-xl flex-col gap-6 p-6 pb-20 sm:px-12 md:py-12"
    >
      <div
        class="grid grid-cols-1 gap-4 text-primary md:grid-cols-[1fr_2fr] lg:grid-cols-[auto_auto_auto_auto_auto]"
      >
        <div
          class="flex flex-col items-center gap-3 md:items-start"
          role="region"
          :aria-label="formatMessage(messages.modrinthInformation)"
        >
          <TextLogo
            aria-hidden="true"
            class="text-logo button-base h-6 w-auto text-contrast lg:h-8"
            @click="developerModeIncrement()"
          />
          <div class="flex flex-wrap justify-center gap-px sm:-mx-2">
            <ButtonStyled
              v-for="(social, index) in socialLinks"
              :key="`footer-social-${index}`"
              circular
              type="transparent"
              :class="{ 'social-disabled': !social.href }"
            >
              <a
                v-if="social.href"
                v-tooltip="formatMessage(social.label)"
                :href="social.href"
                target="_blank"
                :rel="`noopener${social.rel ? ` ${social.rel}` : ''}`"
              >
                <component :is="social.icon" class="h-5 w-5" />
              </a>
              <span v-else v-tooltip="formatMessage(social.label)">
                <component :is="social.icon" class="h-5 w-5" />
              </span>
            </ButtonStyled>
          </div>
          <div class="mt-auto flex flex-wrap justify-center gap-3 md:flex-col">
            <p class="m-0">
              <IntlFormatted :message-id="messages.openSource">
                <template #github-link="{ children }">
                  <a
                    href="https://github.com/AllayMC"
                    class="text-brand hover:underline"
                    target="_blank"
                    rel="noopener"
                  >
                    <component :is="() => children" />
                  </a>
                </template>
              </IntlFormatted>
            </p>
            <p class="m-0">© {{ buildYear }} AllayMC</p>
          </div>
        </div>
        <div class="mt-4 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:contents">
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

.social-disabled {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  opacity: 0.4;
  cursor: not-allowed;
  pointer-events: none;
}
</style>
