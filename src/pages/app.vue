<script setup lang="ts">
import {
  AppleIcon,
  CheckCircleIcon,
  ClipboardCopyIcon,
  DownloadIcon,
  GithubIcon,
  LinuxIcon,
  RocketIcon,
  UpdatedIcon,
  WindowsIcon,
} from '@modrinth/assets'
import AllayLauncherLight from '~/assets/images/app/AllayLauncher_light.webp'
import AllayLauncherDark from '~/assets/images/app/AllayLauncher_dark.webp'
import {
  ButtonStyled,
  commonMessages,
  defineMessages,
  IntlFormatted,
  useVIntl,
} from '@modrinth/ui'

const { formatMessage } = useVIntl()

type OSType = 'Windows' | 'Linux' | 'macOS' | null

const platform = computed<string>(() => {
  if (import.meta.server) {
    const headers = useRequestHeaders()
    return headers['user-agent'] || ''
  } else {
    return navigator.userAgent || ''
  }
})

const os = computed<OSType>(() => {
  if (platform.value.includes('Win')) {
    return 'Windows'
  } else if (platform.value.includes('Mac')) {
    return 'macOS'
  } else if (platform.value.includes('Linux')) {
    return 'Linux'
  } else {
    return null
  }
})

const osName = computed(() => {
  switch (os.value) {
    case 'Windows':
      return formatMessage(messages.windows)
    case 'Linux':
      return formatMessage(messages.linux)
    case 'macOS':
      return formatMessage(messages.macos)
  }
  return ''
})

const messages = defineMessages({
  downloadAllayLauncher: {
    id: 'launcher.hero.download-allay-launcher',
    defaultMessage: 'Download Allay',
  },
  downloadAllayLauncherForOs: {
    id: 'launcher.hero.download-allay-launcher-for-os',
    defaultMessage: 'Download Allay for {os}',
  },
  heroSubtitle: {
    id: 'launcher.hero.subtitle',
    defaultMessage:
      'The launcher for Allay that aims to be small, fast and ready out of the box',
  },
  heroDescription: {
    id: 'launcher.hero.description',
    defaultMessage:
      'A lightweight, fast CLI tool written in C++ that helps you download, update, and manage your Allay server effortlessly.',
  },
  viewOnGithub: {
    id: 'launcher.hero.view-on-github',
    defaultMessage: 'View on GitHub',
  },
  moreDownloadOptions: {
    id: 'launcher.hero.more-download-options',
    defaultMessage: 'Launcher',
  },
  unlikeAnyLauncher: {
    id: 'launcher.features.unlike-any-launcher',
    defaultMessage: 'Simple and powerful',
  },
  youveUsedBefore: {
    id: 'launcher.features.youve-used-before',
    defaultMessage: 'server management',
  },
  featureAutoUpdate: {
    id: 'launcher.features.auto-update.title',
    defaultMessage: 'Auto Update',
  },
  featureAutoUpdateDesc: {
    id: 'launcher.features.auto-update.description',
    defaultMessage:
      'Automatically downloads and updates to the latest version of Allay server.',
  },
  featureJavaChecker: {
    id: 'launcher.features.java-checker.title',
    defaultMessage: 'Java Checker',
  },
  featureJavaCheckerDesc: {
    id: 'launcher.features.java-checker.description',
    defaultMessage:
      'Validates and manages Java runtime to ensure compatibility with Allay server.',
  },
  featureLightweight: {
    id: 'launcher.features.lightweight.title',
    defaultMessage: 'Lightweight',
  },
  featureLightweightDesc: {
    id: 'launcher.features.lightweight.description',
    defaultMessage:
      'Written in C++ for minimal resource usage and maximum performance.',
  },
  downloadOptions: {
    id: 'launcher.download.options-title',
    defaultMessage: 'Download options',
  },
  downloadAllayLauncherTitle: {
    id: 'launcher.download.title',
    defaultMessage: 'Download AllayLauncher',
  },
  downloadDescription: {
    id: 'launcher.download.description',
    defaultMessage:
      'Install AllayLauncher with a single command on your platform.',
  },
  windows: {
    id: 'launcher.download.windows',
    defaultMessage: 'Windows',
  },
  linux: {
    id: 'launcher.download.linux',
    defaultMessage: 'Linux',
  },
  macos: {
    id: 'launcher.download.macos',
    defaultMessage: 'macOS',
  },
  windowsPowershell: {
    id: 'launcher.download.windows-powershell',
    defaultMessage: 'Windows (PowerShell)',
  },
  copy: {
    id: 'launcher.download.copy',
    defaultMessage: 'Copy',
  },
  copied: {
    id: 'launcher.download.copied',
    defaultMessage: 'Copied!',
  },
  openSource: {
    id: 'launcher.features.open-source.title',
    defaultMessage: 'Open source',
  },
  openSourceDescription: {
    id: 'launcher.features.open-source.description',
    defaultMessage:
      'AllayLauncher is fully open source. You can view the source code on our <github-link>GitHub</github-link>!',
  },
})

const title = 'AllayLauncher - AllayHub'
const description =
  'AllayLauncher is a lightweight CLI tool for managing Allay Minecraft Bedrock servers with auto-update, daemon mode, and Java checking.'

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
})

const downloadSection = ref<HTMLElement | null>(null)

const linuxCommand = `wget -qO- https://raw.githubusercontent.com/AllayMC/AllayLauncher/refs/heads/main/scripts/install_linux.sh | bash`
const windowsCommand = `Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/AllayMC/AllayLauncher/refs/heads/main/scripts/install_windows.ps1").Content`
const macosCommand = `wget -qO- https://raw.githubusercontent.com/AllayMC/AllayLauncher/refs/heads/main/scripts/install_macos.sh | bash`

const copiedStates = ref<Record<string, boolean>>({
  linux: false,
  windows: false,
  macos: false,
})

function copyToClipboard(text: string, platform: string) {
  navigator.clipboard.writeText(text)
  copiedStates.value[platform] = true
  setTimeout(() => {
    copiedStates.value[platform] = false
  }, 2000)
}

const downloadOptions = computed(() => {
  const allOptions = [
    {
      id: 'linux',
      os: 'Linux' as OSType,
      label: formatMessage(messages.linux),
      command: linuxCommand,
      icon: LinuxIcon,
    },
    {
      id: 'windows',
      os: 'Windows' as OSType,
      label: formatMessage(messages.windowsPowershell),
      command: windowsCommand,
      icon: WindowsIcon,
    },
    {
      id: 'macos',
      os: 'macOS' as OSType,
      label: formatMessage(messages.macos),
      command: macosCommand,
      icon: AppleIcon,
    },
  ]

  if (os.value) {
    // Move detected OS option to top
    const matched = allOptions.find((opt) => opt.os === os.value)
    const others = allOptions.filter((opt) => opt.os !== os.value)
    return matched ? [matched, ...others] : allOptions
  }

  return allOptions
})

const scrollToSection = () => {
  nextTick(() => {
    if (downloadSection.value) {
      window.scrollTo({
        top: downloadSection.value.offsetTop - 80,
        behavior: 'smooth',
      })
    }
  })
}

const features = computed(() => [
  {
    icon: UpdatedIcon,
    title: formatMessage(messages.featureAutoUpdate),
    description: formatMessage(messages.featureAutoUpdateDesc),
  },
  {
    icon: CheckCircleIcon,
    title: formatMessage(messages.featureJavaChecker),
    description: formatMessage(messages.featureJavaCheckerDesc),
  },
  {
    icon: RocketIcon,
    title: formatMessage(messages.featureLightweight),
    description: formatMessage(messages.featureLightweightDesc),
  },
])
</script>

<template>
  <div>
    <!-- Hero Section -->
    <div class="landing-hero">
      <div
        class="relative mt-4 h-fit w-fit rounded-full bg-highlight-green px-3 py-1 text-sm font-bold text-brand backdrop-blur-lg"
      >
        {{ formatMessage(commonMessages.betaRelease) }}
      </div>
      <h1 class="main-header max-w-[60rem]">
        {{
          os
            ? formatMessage(messages.downloadAllayLauncherForOs, { os: osName })
            : formatMessage(messages.downloadAllayLauncher)
        }}
      </h1>
      <h2 class="main-subheader">
        {{ formatMessage(messages.heroSubtitle) }}
      </h2>
      <p class="hero-description">
        {{ formatMessage(messages.heroDescription) }}
      </p>
      <div class="button-group">
        <ButtonStyled color="brand" size="large">
          <a
            href="https://github.com/AllayMC/AllayLauncher"
            target="_blank"
            rel="noopener"
          >
            <GithubIcon />
            {{ formatMessage(messages.viewOnGithub) }}
          </a>
        </ButtonStyled>
        <ButtonStyled type="transparent" size="large">
          <button class="outline-button" @click="scrollToSection">
            <DownloadIcon />
            {{ formatMessage(messages.moreDownloadOptions) }}
          </button>
        </ButtonStyled>
      </div>
      <div class="launcher-screenshot">
        <img
          :src="AllayLauncherLight"
          alt="AllayLauncher"
          class="light-mode-img"
        />
        <img
          :src="AllayLauncherDark"
          alt="AllayLauncher"
          class="dark-mode-img"
        />
      </div>
      <div class="bottom-transition" />
    </div>

    <!-- Features Section -->
    <div class="features">
      <h1 class="subheader">
        {{ formatMessage(messages.unlikeAnyLauncher) }} <br />
        {{ formatMessage(messages.youveUsedBefore) }}
      </h1>
      <div class="feature-grid">
        <div
          v-for="feature in features"
          :key="feature.title"
          class="feature gradient-border"
        >
          <div class="feature-icon">
            <component :is="feature.icon" />
          </div>
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.description }}</p>
        </div>
      </div>
      <div class="feature-row">
        <div class="point">
          <div class="title">
            <GithubIcon class="point-icon" />
            <h3>{{ formatMessage(messages.openSource) }}</h3>
          </div>
          <div class="description">
            <IntlFormatted :message-id="messages.openSourceDescription">
              <template #github-link="{ children }">
                <a
                  href="https://github.com/AllayMC/AllayLauncher"
                  rel="noopener"
                  target="_blank"
                >
                  <component :is="() => children" />
                </a>
              </template>
            </IntlFormatted>
          </div>
        </div>
      </div>
    </div>

    <!-- Download Section -->
    <div ref="downloadSection" class="footer">
      <div class="section-badge">
        {{ formatMessage(messages.downloadOptions) }}
      </div>
      <div class="section-subheader">
        <div class="section-subheader-title">
          {{ formatMessage(messages.downloadAllayLauncherTitle) }}
        </div>
        <div class="section-subheader-description">
          {{ formatMessage(messages.downloadDescription) }}
        </div>
      </div>
      <div class="download-section">
        <template v-for="(option, index) in downloadOptions" :key="option.id">
          <div v-if="index > 0" class="divider" />
          <div class="download-card gradient-border">
            <div class="title">
              <component :is="option.icon" class="os-icon" />
              {{ option.label }}
            </div>
            <div class="command-block">
              <code>{{ option.command }}</code>
              <button
                class="copy-button"
                :class="{ copied: copiedStates[option.id] }"
                @click="copyToClipboard(option.command, option.id)"
              >
                <ClipboardCopyIcon v-if="!copiedStates[option.id]" />
                <CheckCircleIcon v-else />
                {{
                  copiedStates[option.id]
                    ? formatMessage(messages.copied)
                    : formatMessage(messages.copy)
                }}
              </button>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.launcher-screenshot {
  .light-mode-img {
    display: block;
  }

  .dark-mode-img {
    display: none;
  }
}

html.dark-mode,
html.oled-mode,
html.retro-mode {
  .launcher-screenshot {
    .light-mode-img {
      display: none;
    }

    .dark-mode-img {
      display: block;
    }
  }
}
</style>

<style scoped lang="scss">
.launcher-screenshot {
  margin-top: 3rem;
  max-width: 60rem;
  width: 100%;
  padding: 0 1rem;

  img {
    width: 100%;
    height: auto;
    border-radius: var(--size-rounded-card);
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }
}

.landing-hero {
  position: relative;
  background: var(--color-bg);
  padding: 6rem 1rem 4rem 1rem;
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  flex-direction: column;

  .main-header {
    font-size: clamp(2.5rem, 8vw, 5.25rem);
    font-weight: 600;
    line-height: 100%;
    margin: 1rem 0 2rem;
    color: var(--color-contrast);
  }

  .main-subheader {
    font-size: clamp(1rem, 3vw, 1.625rem);
    color: var(--color-brand);
    margin: 0 0 1rem;
    font-weight: 600;
    line-height: 125%;
  }

  .hero-description {
    font-size: 1.125rem;
    color: var(--color-text-secondary);
    margin: 0 0 2rem;
    line-height: 1.6;
    max-width: 40rem;
  }

  .button-group {
    display: flex;
    justify-content: center;
    gap: 1rem;
    flex-wrap: wrap;

    a,
    button {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      svg {
        width: 1.25rem;
        height: 1.25rem;
      }
    }

    .outline-button {
      background: transparent;
      border: 1px solid var(--color-divider);
      color: var(--color-contrast);

      &:hover {
        background: var(--color-button-bg);
      }
    }
  }
}

.subheader {
  font-size: clamp(1.75rem, 5vw, 3.5rem);
  font-weight: 600;
  line-height: 100%;
  margin: 0 auto;
  padding: 0 4rem 4rem;
  text-align: center;
  color: var(--color-contrast);
}

.features {
  position: relative;
  width: 100%;
  background: var(--color-raised-bg);
  padding: 4rem 1rem;
  align-content: center;
  justify-content: center;
  display: flex;
  flex-direction: column;

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    max-width: 1000px;
    margin: 0 auto 3rem;
    padding: 0 1rem;

    .feature {
      background: var(--color-bg);
      border-radius: var(--size-rounded-card);
      padding: 1.5rem;
      text-align: center;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        border-radius: var(--size-rounded-card);
        padding: 1px;
        background: linear-gradient(
          135deg,
          var(--color-brand) 0%,
          transparent 50%,
          var(--color-brand) 100%
        );
        -webkit-mask:
          linear-gradient(#fff 0 0) content-box,
          linear-gradient(#fff 0 0);
        -webkit-mask-composite: xor;
        mask-composite: exclude;
        opacity: 0.3;
        pointer-events: none;
      }

      .feature-icon {
        width: 3rem;
        height: 3rem;
        margin: 0 auto 1rem;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--color-brand-highlight);
        border-radius: 50%;
        color: var(--color-brand);

        svg {
          width: 1.5rem;
          height: 1.5rem;
        }
      }

      h3 {
        font-size: 1.125rem;
        font-weight: 600;
        margin: 0 0 0.5rem;
        color: var(--color-contrast);
      }

      p {
        font-size: 0.9375rem;
        color: var(--color-text-secondary);
        margin: 0;
        line-height: 1.5;
      }
    }
  }

  .feature-row {
    display: flex;
    justify-content: center;
    gap: 3rem;
    flex-wrap: wrap;
    max-width: 1000px;
    margin: 0 auto;
    padding: 0 1rem;

    .point {
      text-align: center;
      max-width: 20rem;

      .title {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
        margin-bottom: 0.5rem;

        .point-icon {
          width: 1.5rem;
          height: 1.5rem;
          color: var(--color-brand);
        }

        h3 {
          margin: 0;
          font-size: 1.125rem;
          font-weight: 600;
          color: var(--color-contrast);
        }
      }

      .description {
        color: var(--color-text-secondary);
        font-size: 0.9375rem;
        line-height: 1.5;

        a {
          color: var(--color-brand);
          text-decoration: underline;

          &:hover {
            opacity: 0.8;
          }
        }
      }
    }
  }
}

.footer {
  background: var(--color-bg);
  padding: 4rem 1rem;
  display: flex;
  flex-direction: column;
  align-items: center;

  .section-badge {
    display: inline-flex;
    padding: 0.5rem 1rem;
    background: var(--color-brand-highlight);
    color: var(--color-brand);
    border-radius: 2rem;
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
  }

  .section-subheader {
    text-align: center;
    margin-bottom: 2rem;

    .section-subheader-title {
      font-size: 2rem;
      font-weight: 700;
      color: var(--color-contrast);
      margin-bottom: 0.5rem;
    }

    .section-subheader-description {
      color: var(--color-text-secondary);
      font-size: 1.125rem;
    }
  }

  .download-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
    max-width: 800px;

    .divider {
      height: 1px;
      background: var(--color-divider);
    }

    .download-card {
      background: var(--color-raised-bg);
      border-radius: var(--size-rounded-card);
      padding: 1.5rem;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        border-radius: var(--size-rounded-card);
        padding: 1px;
        background: linear-gradient(
          135deg,
          var(--color-brand) 0%,
          transparent 50%,
          var(--color-brand) 100%
        );
        -webkit-mask:
          linear-gradient(#fff 0 0) content-box,
          linear-gradient(#fff 0 0);
        -webkit-mask-composite: xor;
        mask-composite: exclude;
        opacity: 0.3;
        pointer-events: none;
      }

      .title {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--color-contrast);
        margin-bottom: 1rem;

        .os-icon {
          width: 1.5rem;
          height: 1.5rem;
        }
      }

      .command-block {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        background: var(--color-bg);
        border-radius: var(--radius-lg);
        padding: 1rem;

        @media (max-width: 640px) {
          flex-direction: column;
        }

        code {
          flex: 1;
          font-family: var(--font-mono, monospace);
          font-size: 0.875rem;
          color: var(--color-text);
          word-break: break-all;
          line-height: 1.5;
        }

        .copy-button {
          display: flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.5rem 1rem;
          background: var(--color-button-bg);
          border: 1px solid var(--color-divider);
          border-radius: var(--radius-md);
          color: var(--color-text);
          font-size: 0.875rem;
          cursor: pointer;
          transition: all 0.2s;
          flex-shrink: 0;

          svg {
            width: 1rem;
            height: 1rem;
          }

          &:hover {
            background: var(--color-raised-bg);
          }

          &.copied {
            background: var(--color-brand-highlight);
            color: var(--color-brand);
            border-color: var(--color-brand);
          }
        }
      }
    }
  }
}
</style>
